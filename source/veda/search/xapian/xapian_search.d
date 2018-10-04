/**
 * VQL executor
 */

module veda.search.xapian.xapian_search;

private
{
    import std.string, std.array, std.stdio, std.conv, std.datetime, std.json, std.outbuffer, core.stdc.string, std.concurrency;
    import veda.util.container, veda.common.logger, veda.core.util.utils;
    import veda.core.common.context, veda.core.common.define, veda.core.common.know_predicates, veda.common.type;
    import veda.search.common.isearch, veda.search.common.vel, veda.search.xapian.xapian_reader;
    import veda.onto.individual;
}

static const int XAPIAN = 2;

class XapianSearch : Search
{
    private string[]     sections;
    private bool[]       section_is_found;
    private string[]     found_sections;

    private Context      context;
    private XapianReader xr;
    private Logger       log;

    this(Context _context)
    {
        sections         = [ "return", "filter", "sort", "render", "authorize", "source" ];
        found_sections   = new string[ sections.length ];
        section_is_found = new bool[ sections.length ];

        context = _context;
        log     = context.get_logger();
        xr      = new XapianReader(_context);
    }

    public bool close_db()
    {
        return xr.close_dbs();
    }

    public void reopen_db()
    {
        xr.reopen_dbs();
    }

    public int query(string user_uri, string filter, string freturn, string sort, int top, int limit,
                     ref Individual[] individuals, OptAuthorize op_auth, bool trace)
    {
        int                       res_count;

        void delegate(string uri) dg;
        void collect_subject(string uri)
        {
            if (uri is null)
            {
                individuals = individuals.init;
                return;
            }

            Individual individual = Individual();

            context.get_storage().get_obj_from_individual_storage(uri, individual);

            if (individual.getStatus() == ResultCode.Not_Found)
            {
                log.trace("ERR! FT:get Unable to find the object [%s] it should be, query=[%s]", uri, filter);
            }
            else if (individual.getStatus() == ResultCode.OK)
            {
                individuals ~= individual;
            }
            else
            {
                log.trace("ERR!: FT:get invalid individual=%s, status=%s, query=%s", uri, individual.getStatus(), filter);
            }
        }
        dg = &collect_subject;

        SearchResult sr = xr.query(user_uri, filter, freturn, sort, 0, top, limit, dg, op_auth, null, trace);
        res_count = sr.count;

        return res_count;
    }

    public SearchResult query(string user_uri, string filter, string freturn, string sort, int from, int top, int limit,
                              void delegate(string uri) prepare_element_event,
                              OptAuthorize op_auth, bool trace)
    {
        string[]                  res;

        void delegate(string uri) dg;
        void collect_subject(string uri)
        {
            if (uri is null)
            {
                res = res.init;
                return;
            }
            res ~= uri;
        }
        dg = &collect_subject;

        SearchResult sr = xr.query(user_uri, filter, freturn, sort, from, top, limit, dg, op_auth, prepare_element_event, trace);

        if (sr.result_code == ResultCode.OK)
            sr.result = res;

        return sr;
    }

    public int query(string user_uri, string query_str, ref Individual[] res, OptAuthorize op_auth, bool trace)
    {
        split_on_section(query_str);
        int top = 10000;
        try
        {
            if (found_sections[ RENDER ] !is null && found_sections[ RENDER ].length > 0)
                top = parse!int (found_sections[ RENDER ]);
        } catch (Exception ex)
        {
        }
        int limit = 10000;
        try
        {
            if (found_sections[ AUTHORIZE ] !is null && found_sections[ AUTHORIZE ].length > 0)
                limit = parse!int (found_sections[ AUTHORIZE ]);
        } catch (Exception ex)
        {
        }
        string sort;
        if (section_is_found[ SORT ] == true)
            sort = found_sections[ SORT ];
        int type_source = XAPIAN;
        if (found_sections[ SOURCE ] == "xapian")
            type_source = XAPIAN;

        string dummy;
        double d_dummy;
        int    res_count;

        if (type_source == XAPIAN)
        {
            void delegate(string uri) dg;
            void collect_subject(string uri)
            {
                if (uri is null)
                {
                    res = res.init;
                    return;
                }

                Individual ind;
                context.get_storage().get_obj_from_individual_storage(uri, ind);

                if (ind.getStatus() == ResultCode.Not_Found)
                {
                    log.trace("ERR! Unable to find the object [%s] it should be, query=[%s]", uri, query_str);
                }
                else if (ind.getStatus() == ResultCode.OK)
                {
                    res ~= ind;
                }
                else
                {
                    context.reopen_ro_individuals_storage_db();

                    context.get_storage().get_obj_from_individual_storage(uri, ind);

                    if (ind.getStatus() == ResultCode.OK)
                    {
                        res ~= ind;
                    }
                    else
                    {
                        log.trace("ERR! FT:get attempt 2, invalid individual=%s", uri);
                    }
                }

                //                log.trace("ERR!: FT:get invalid individual=%s, status=%s, query=%s", uri, individual.getStatus(), query_str);
            }


            dg = &collect_subject;

            SearchResult sr = xr.query(user_uri, found_sections[ FILTER ], found_sections[ RETURN ], sort, 0, top, limit, dg, op_auth, null, trace);
            res_count = sr.count;
        }

        return res_count;
    }

    private void split_on_section(string query)
    {
        section_is_found[] = false;
        if (query is null)
            return;

        for (int pos = 0; pos < query.length; pos++)
        {
            for (int i = 0; i < sections.length; i++)
            {
                char cc = query[ pos ];
                if (section_is_found[ i ] == false)
                {
                    found_sections[ i ] = null;

                    int j     = 0;
                    int t_pos = pos;
                    while (sections[ i ][ j ] == cc && t_pos < query.length && j < sections[ i ].length)
                    {
                        j++;
                        t_pos++;

                        if (t_pos >= query.length || j >= sections[ i ].length)
                            break;

                        cc = query[ t_pos ];
                    }

                    if (j == sections[ i ].length)
                    {
                        pos = t_pos;
                        // нашли
                        section_is_found[ i ] = true;

                        while (query[ pos ] != '{' && pos < query.length)
                            pos++;
                        pos++;

                        while (query[ pos ] == ' ' && pos < query.length)
                            pos++;

                        int bp = pos;
                        while (query[ pos ] != '}' && pos < query.length)
                            pos++;
                        pos--;

                        while (query[ pos ] == ' ' && pos > bp)
                            pos--;
                        int ep = pos + 1;

                        found_sections[ i ] = query[ bp .. ep ];
                    }
                }
            }
        }
    }
}