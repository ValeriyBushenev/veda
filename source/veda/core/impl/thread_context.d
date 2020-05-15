/**

 * Межмодульное API - Реализация
 */

module veda.core.impl.thread_context;

private
{
    import core.thread, std.stdio, std.format, std.datetime, std.concurrency, std.conv, std.outbuffer, std.string, std.file, std.path,
           std.json, std.regex, std.uuid;
    import properd;
    import veda.common.logger, veda.core.util.utils, veda.onto.bj8individual.individual8json;
    import veda.common.type, veda.core.common.type, veda.core.common.define, veda.core.common.context;
    import veda.onto.onto, veda.onto.individual, veda.onto.resource, veda.storage.common, veda.storage.storage;
    import veda.search.common.isearch, veda.util.module_info, veda.common.logger;
    import veda.authorization.authorization;
}

/// реализация интерфейса Context
class PThreadContext : Context
{
    private Onto          onto;

    public string         name;

    private Search        _vql;
    public Storage        storage;
    private Authorization az;

    private long          local_last_update_time;
    private Individual    node = Individual.init;
    public string         node_id;

    private bool          API_ready = true;
    public string         main_module_url;
    public Logger         log;

    Ticket *[ string ] user_of_ticket;
    long last_ticket_manager_op_id = 0;

    public Ticket *get_systicket_from_storage(){
        Individual indv_systicket_link;

        storage.get_tickets_storage_r().get_individual("systicket", indv_systicket_link);

        string systicket_id;

        if (indv_systicket_link.getStatus == ResultCode.Ok) {
            systicket_id = indv_systicket_link.getFirstLiteral("v-s:resource");
        }else  {
            log.trace("SYSTICKET NOT FOUND");
        }

        return get_ticket(systicket_id, false);
    }

    public Ticket create_new_ticket(string user_login, string user_id, string duration, string ticket_id, bool is_trace = false){
        if (is_trace)
            log.trace("create_new_ticket, ticket__accessor=%s", user_id);

        Ticket     ticket;
        Individual new_ticket;

        ticket.result = ResultCode.FailStore;

        Resources type = [ Resource("ticket:ticket") ];

        new_ticket.resources[ "rdf:type" ] = type;

        if (ticket_id !is null && ticket_id.length > 0)
            new_ticket.uri = ticket_id;
        else{
            UUID new_id = randomUUID();
            new_ticket.uri = new_id.toString();
        }

        new_ticket.resources[ "ticket:login" ] ~= Resource(user_login);
        new_ticket.resources[ "ticket:accessor" ] ~= Resource(user_id);
        new_ticket.resources[ "ticket:when" ] ~= Resource(getNowAsString());
        new_ticket.resources[ "ticket:duration" ] ~= Resource(duration);

        return ticket;
    }

    public Ticket *get_ticket(string ticket_id, bool is_trace, bool is_systicket = false){
        //StopWatch sw; sw.start;

        try
        {
            Ticket *tt;
            if (ticket_id is null || ticket_id == "" || ticket_id == "systicket")
                ticket_id = "guest";

            tt = user_of_ticket.get(ticket_id, null);

            if (tt is null) {
                string when     = null;
                int    duration = 0;

                MInfo  mi = get_info(MODULE.ticket_manager);

                //log.trace ("last_ticket_manager_op_id=%d, mi.op_id=%d,  mi.committed_op_id=%d", last_ticket_manager_op_id, mi.op_id, mi.committed_op_id);
                if (last_ticket_manager_op_id < mi.op_id) {
                    last_ticket_manager_op_id = mi.op_id;
                    storage.reopen_ro_ticket_manager_db();
                }

                Individual ticket;
                storage.get_tickets_storage_r().get_individual(ticket_id, ticket);

                if (ticket.getStatus() == ResultCode.Ok) {
                    tt = new Ticket;
                    subject2Ticket(ticket, tt);
                    tt.result               = ResultCode.Ok;
                    user_of_ticket[ tt.id ] = tt;
                }else if (ticket.getStatus() == ResultCode.NotFound) {
                    tt        = new Ticket;
                    tt.result = ResultCode.TicketNotFound;

                    if (is_trace)
                        log.trace("тикет не найден в базе, id=%s", ticket_id);
                }else  {
                    tt        = new Ticket;
                    tt.result = ResultCode.UnprocessableEntity;
                    log.trace("ERR! storage.get_ticket, invalid individual, uri=%s, errcode=%s", ticket_id, ticket.getStatus());
                }
            }else  {
                if (is_trace)
                    log.trace("тикет нашли в кеше, id=%s, end_time=%d", tt.id, tt.end_time);

                SysTime now = Clock.currTime();
                if (now.stdTime >= tt.end_time && !is_systicket) {
                    log.trace("ticket %s expired, user=%s, start=%s, end=%s, now=%s", tt.id, tt.user_uri, SysTime(tt.start_time,
                                                                                                                  UTC()).toISOExtString(),
                              SysTime(tt.end_time, UTC()).toISOExtString(), now.toISOExtString());

                    if (ticket_id == "guest") {
                        Ticket guest_ticket = create_new_ticket("guest", "cfg:Guest", "900000000", "guest");
                        tt = &guest_ticket;
                    }else  {
                        tt        = new Ticket;
                        tt.id     = "?";
                        tt.result = ResultCode.TicketExpired;
                    }
                    return tt;
                }else  {
                    tt.result = ResultCode.Ok;
                }

                if (is_trace)
                    log.trace("ticket: %s", *tt);
            }
            return tt;
        }
        finally
        {
        }
    }

    private ModuleInfoFile[ MODULE ] info_r__2__pmodule;
    public MInfo get_info(MODULE module_id){
        ModuleInfoFile mdif = info_r__2__pmodule.get(module_id, null);

        if (mdif is null) {
            mdif                            = new ModuleInfoFile(text(module_id), log, OPEN_MODE.READER);
            info_r__2__pmodule[ module_id ] = mdif;
        }
        MInfo info = mdif.get_info();
        return info;
    }

    Storage get_storage(){
        return storage;
    }

    public Logger get_logger(){
        return log;
    }

    version (isModule)
    {
        import kaleidic.nanomsg.nano;

        private int sock_main_module = -1;

        private int get_sock_2_main_module(){
            if (sock_main_module >= 0)
                return sock_main_module;

            sock_main_module = nn_socket(AF_SP, NN_REQ);
            if (sock_main_module < 0) {
                log.trace("ERR! cannot create socket");
                return -1;
            }else if (nn_connect(sock_main_module, cast(char *)main_module_url) < 0) {
                log.trace("ERR! cannot connect socket to %s", main_module_url);
                return -1;
            }else  {
                log.trace("success connect %s", main_module_url);
                return sock_main_module;
            }
        }

        private OpResult[] reqrep_json_2_main_module(ref JSONValue jreq){
            string req = jreq.toString();

            return reqrep_binobj_2_main_module(req);
        }

        private OpResult[] reqrep_binobj_2_main_module(string req){
            string     rep;
            int        res;

            OpResult[] ress;

            try
            {
                int sock = get_sock_2_main_module();

                if (sock >= 0) {
                    char *buf = cast(char *)0;

                    res = nn_send(sock, cast(char *)req, req.length, 0);

                    if (res < 0) {
                        log.trace("ERR! N_CHANNEL: send: err=%s", fromStringz(nn_strerror(nn_errno())));
                        log.trace("N_CHANNEL send (%s)", req);
                    }


                    for (int attempt = 0; attempt < 10; attempt++) {
                        res = nn_recv(sock, &buf, NN_MSG, 0);

                        if (res < 0) {
                            log.trace("ERR! N_CHANNEL: recv: err=%s", fromStringz(nn_strerror(nn_errno())));
                        }

                        if (res > 0 || res == -1 && nn_errno() != 4)
                            break;

                        log.trace("ERR! N_CHANNEL: repeat recv, attempt=%d", attempt + 1);
                    }


                    if (res >= 0) {
                        int bytes = res;

                        rep = to!string(buf);
                        //log.trace("N_CHANNEL recv (%s)", rep);

                        JSONValue jres = parseJSON(rep);

                        if (jres[ "type" ].str == "OpResult") {
                            if ("data" in jres) {
                                JSONValue data = jres[ "data" ];
                                if (data !is JSONValue.init) {
                                    foreach (ii; data.array) {
                                        OpResult ores;

                                        ores.op_id  = ii[ "op_id" ].integer;
                                        ores.result = cast(ResultCode)ii[ "result" ].integer;
                                        ress ~= ores;
                                    }
                                }
                            }else  {
                                OpResult ores;
                                ores.op_id  = jres[ "op_id" ].integer;
                                ores.result = cast(ResultCode)jres[ "result" ].integer;
                                ress ~= ores;
                            }
                        }

                        nn_freemsg(buf);
                    }
                }else  {
                    log.trace("ERR! N_CHANNEL: invalid socket");
                }

                if (ress.length == 0) {
                    log.trace("ERR! reqrep_json_2_main_module, empty result, sock=%d", sock);
                    log.trace("req: (%s)", req);
                    log.trace("rep: (%s)", rep);
                    OpResult ores;
                    ores.op_id  = -1;
                    ores.result = ResultCode.InternalServerError;
                    return [ ores ];
                }

                return ress;
            }
            catch (Throwable tr)
            {
                log.trace("ERR! reqrep_json_2_main_module, %s", tr.info);
                log.trace("req: %s", req);
                log.trace("rep: %s", rep);

                if (ress.length == 0) {
                    OpResult ores;
                    ores.op_id  = -1;
                    ores.result = ResultCode.InternalServerError;
                    return [ ores ];
                }

                return ress;
            }
        }
    }

    public string get_config_uri(){
        return node_id;
    }

    bool isReadyAPI(){
        return API_ready;
    }

    public Ticket sys_ticket(bool is_new = false){
        Ticket ticket = get_global_systicket();

        version (isModule)
        {
            ticket = *(get_systicket_from_storage());
            set_global_systicket(ticket);
        }

        return ticket;
    }

    public Individual get_configuration(){
        if (node == Individual.init && node_id !is null) {
            this.reopen_ro_individuals_storage_db();
            node = get_individual(node_id);
            if (node.getStatus() != ResultCode.Ok)
                node = Individual.init;
        }
        return node;
    }

    private long local_count_onto_update = -1;

    public Onto get_onto(){
        if (onto !is null) {
            long g_count_onto_update = get_count_onto_update();
            if (g_count_onto_update > local_count_onto_update) {
                local_count_onto_update = g_count_onto_update;
                onto_load();
            }
        }else  {
            onto = new Onto(this.log);
            onto_load();
        }

        return onto;
    }

    public void onto_load(){
        std.datetime.stopwatch.StopWatch sw1;

        sw1.start();

        reopen_ro_individuals_storage_db();
        reopen_ro_fulltext_indexer_db();

        Ticket       sticket       = sys_ticket();
        Individual[] l_individuals = get_individuals_via_query(
                                                               sticket.user_uri,
                                                               "'rdf:type' === 'rdfs:Class' || 'rdf:type' === 'rdf:Property' || 'rdf:type' === 'owl:Class' || 'rdf:type' === 'owl:ObjectProperty' || 'rdf:type' === 'owl:DatatypeProperty'",
                                                               OptAuthorize.NO, 10000, 10000);

        sw1.stop();

        log.trace_log_and_console("[%s] load onto, count individuals: %d, time=%d µs", get_name, l_individuals.length, sw1.peek.total !"usecs");
        onto.load(l_individuals);
    }

    public string get_name(){
        return name;
    }

    // *************************************************** external api *********************************** //

    // /////////////////////////////////////////////////////// TICKET //////////////////////////////////////////////

    public bool is_ticket_valid(string ticket_id){
        Ticket *ticket = get_ticket(ticket_id, false);

        if (ticket is null)
            return false;

        SysTime now = Clock.currTime();
        if (now.stdTime < ticket.end_time)
            return true;

        return false;
    }

    // //////////////////////////////////////////// INDIVIDUALS IO /////////////////////////////////////
    public Individual[] get_individuals_via_query(string user_uri, string query_str, OptAuthorize op_auth, int top = 10, int limit = 10000){
        Individual[] res;

        try
        {
            if (query_str.indexOf("==") > 0 || query_str.indexOf("&&") > 0 || query_str.indexOf("||") > 0) {
            }else  {
                query_str = "'*' == '" ~ query_str ~ "'";
            }

            _vql.query(user_uri, query_str, null, null, top, limit, op_auth, false, res);
            return res;
        }
        finally
        {
            //log.trace("get_individuals_via_query: end, query_str=%s, result=%s", query_str, res);
        }
    }

    public void set_vql(Search in_vql){
        _vql = in_vql;
    }

    public Search get_vql(){
        return _vql;
    }

    public Authorization get_az(){
        return az;
    }

    public void set_az(Authorization in_az){
        az = in_az;
    }

    public void reopen_ro_fulltext_indexer_db(){
        if (_vql !is null)
            _vql.reopen_db();
    }

    public void reopen_ro_individuals_storage_db(){
        if (storage !is null)
            storage.get_inividuals_storage_r().reopen();
    }

    // ////////// external ////////////

    public SearchResult get_individuals_ids_via_query(string user_uri, string query_str, string sort_str, string db_str, int from, int top, int limit,
                                                      OptAuthorize op_auth, bool trace){
        SearchResult sr;

        if ((query_str.indexOf("==") > 0 || query_str.indexOf("&&") > 0 || query_str.indexOf("||") > 0) == false)
            query_str = "'*' == '" ~ query_str ~ "'";

        sr = _vql.query(user_uri, query_str, sort_str, db_str, from, top, limit, op_auth, trace);

        return sr;
    }

    public Individual get_individual(string uri){
        Individual individual = Individual.init;

        try
        {
            storage.get_obj_from_individual_storage(uri, individual);
            return individual;
        }
        finally
        {
//            log.trace("get_individual: end, uri=%s", uri);
        }
    }

    public OpResult update(string src, long tnx_id, Ticket *ticket, INDV_OP cmd, Individual *indv, string event_id, MODULES_MASK assigned_subsystems,
                           OptAuthorize opt_request){
        //log.trace("[%s] add_to_transaction: %s %s", name, text(cmd), *indv);

        //StopWatch sw; sw.start;

        OpResult res = OpResult(ResultCode.FailStore, -1);

        try
        {
            if (indv !is null && (indv.uri is null || indv.uri.length < 2)) {
                res.result = ResultCode.InvalidIdentifier;
                return res;
            }
            if (indv is null || (cmd != INDV_OP.REMOVE && indv.resources.length == 0)) {
                res.result = ResultCode.NoContent;
                return res;
            }

            version (isModule)
            {
                //log.trace("[%s] add_to_transaction: isModule", name);

                string scmd;

                if (cmd == INDV_OP.PUT)
                    scmd = "put";
                else if (cmd == INDV_OP.ADD_IN)
                    scmd = "add_to";
                else if (cmd == INDV_OP.SET_IN)
                    scmd = "set_in";
                else if (cmd == INDV_OP.REMOVE_FROM)
                    scmd = "remove_from";
                else if (cmd == INDV_OP.REMOVE)
                    scmd = "remove";

                JSONValue req_body;
                req_body[ "function" ]            = scmd;
                req_body[ "ticket" ]              = ticket.id;
                req_body[ "individuals" ]         = [ individual_to_json(*indv) ];
                req_body[ "assigned_subsystems" ] = assigned_subsystems;
                req_body[ "event_id" ]            = event_id;
                req_body[ "src" ]                 = src;
                req_body[ "tnx_id" ]              = tnx_id;

                //log.trace("[%s] add_to_transaction: (isModule), req=(%s)", name, req_body.toString());

                res = reqrep_json_2_main_module(req_body)[ 0 ];
                //log.trace("[%s] add_to_transaction: (isModule), rep=(%s)", name, res);
            }

            return res;
        }
        finally
        {
            if (res.result != ResultCode.Ok)
                log.trace("ERR! update: no store individual: errcode=[%s], ticket=[%s] indv=[%s]", text(res.result),
                          ticket !is null ? text(*ticket) : "null",
                          indv !is null ? text(*indv) : "null");

            //   log.trace("[%s] add_to_transaction [%s] = %s", name, indv.uri, res);
        }
    }

}
