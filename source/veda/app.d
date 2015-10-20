import std.conv, std.stdio, std.file;
import vibe.d;
import properd;
import veda.pacahon_driver;
import veda.storage_rest;
import veda.onto.individual, veda.onto.resource, veda.core.context, veda.core.define;

void view_error(HTTPServerRequest req, HTTPServerResponse res, HTTPServerErrorInfo error)
{
    res.renderCompat!("view_error.dt",
                      HTTPServerRequest, "req",
                      HTTPServerErrorInfo, "error")(req, error);
}

void uploadFile(HTTPServerRequest req, HTTPServerResponse res)
{
    string filename;

    try
    {
        auto pf = "file" in req.files;

        enforce(pf !is null, "No file uploaded!");

        auto pt = "path" in req.form;
        auto nm = "uri" in req.form;
        if (pt !is null && nm !is null)
        {
            string pts = cast(string)*pt;
            filename = cast(string)*nm;

            string[] ptspc = pts.split('/');

            string   np = "./data/files/";
            foreach (it; ptspc)
            {
                np ~= it ~ "/";
                try
                {
                    mkdir(np);
                }
                catch (Exception ex)
                {
                }
            }

            Path path = Path("data/files/" ~ pts ~ "/") ~filename;

            try moveFile(pf.tempPath, path);
            catch (Exception e) {
//                logWarn("Failed to move file to destination folder: %s", e.msg);
//                logInfo("Performing copy+delete instead.");
                copyFile(pf.tempPath, path);
            }

            res.writeBody("File uploaded!", "text/plain");
        }
    }
    catch (Throwable ex)
    {
        writeln("Ex!: ", __FUNCTION__, ":", text(__LINE__), ", ", ex.msg, ", filename:", filename);
    }
}


shared static this()
{
    import etc.linux.memoryerror;
    static if (is (typeof(registerMemoryErrorHandler)))
        registerMemoryErrorHandler();

    import vibe.core.args;
    string role;
    ushort listener_http_port = 8081;
    string write_storage_node = "http://127.0.0.1:8080";

    readOption("role", &role, "set role, if role is empty, then ignore params: [listener_http_port] and [write_storage_node]");
    readOption("listener_http_port", &listener_http_port, "default: 8081");
    readOption("write_storage_node", &write_storage_node, "default: http://127.0.0.1:8080");

    string[ string ] properties;

    try
    {
        properties = readProperties("./veda.properties");
    }
    catch (Exception ex)
    {
    }

    string node_id;

    if (role is null)
    {
        node_id            = properties.as!(string)("node_id");
        listener_http_port = 0;
        write_storage_node = null;
    }
    else
    {
        proccess_name = role;
    }

//    http_port    = properties.as!(string)("node_id");
//    count_thread = properties.as!(int)("count_thread");
//    int checktime_onto_files = properties.as!(int)("checktime_onto_files");

//    if (checktime_onto_files < 1)
//        checktime_onto_files = 30;
    veda.core.context.Context core_context;

    core_context = veda.core.server.init_core(node_id, role, listener_http_port, write_storage_node);
    if (core_context is null)
    {
        writeln("ERR: Veda core has not been initialized");
        return;
    }

    Ticket                sticket      = core_context.sys_ticket();
    ushort                count_thread = 1;
    std.concurrency.Tid[] pool;
    for (int i = 0; i < count_thread; i++)
    {
        pool ~= std.concurrency.spawnLinked(&core_thread, node_id, write_storage_node);
        core.thread.Thread.sleep(dur!("msecs")(10));
    }

    if (role !is null)
    {
        start_http_listener(core_context, pool, listener_http_port);
    }
    else
    {
        Individual node = core_context.get_individual(&sticket, node_id);

        count_thread = cast(ushort)node.getFirstInteger("vsrv:count_thread", 4);

        Resources listeners = node.resources.get("vsrv:listener", Resources.init);
        foreach (listener_uri; listeners)
        {
            Individual connection = core_context.get_individual(&sticket, listener_uri.uri);

            Resource   transport = connection.getFirstResource("vsrv:transport");
            if (transport != Resource.init)
            {
                if (transport.data() == "http")
                {
                    ushort http_port = cast(ushort)connection.getFirstInteger("vsrv:port", 8080);
                    start_http_listener(core_context, pool, http_port);
                }
            }
        }
    }
}

void start_http_listener(Context core_context, ref std.concurrency.Tid[] pool, ushort http_port)
{
    VedaStorageRest vsr = new VedaStorageRest(pool, core_context);

    auto            settings = new HTTPServerSettings;

    settings.port           = http_port;
    settings.maxRequestSize = 1024 * 1024 * 1000;
    //settings.bindAddresses = ["::1", "127.0.0.1", "172.17.35.148"];
    //settings.bindAddresses = ["127.0.0.1"];
    settings.errorPageHandler = toDelegate(&view_error);
    //settings.options = HTTPServerOption.parseURL|HTTPServerOption.distribute;

    auto router = new URLRouter;
    router.get("/files/*", &vsr.fileManager);
    router.get("*", serveStaticFiles("public"));
    router.get("/", serveStaticFile("public/index.html"));
    router.get("/tests", serveStaticFile("public/tests.html"));
    router.post("/files", &uploadFile);

    registerRestInterface(router, vsr);

    logInfo("============ROUTES=============");
    auto routes = router.getAllRoutes();
    logInfo("GET:");
    foreach (route; routes)
    {
        if (route.method == HTTPMethod.GET)
            logInfo(route.pattern);
    }

    logInfo("PUT:");
    foreach (route; routes)
    {
        if (route.method == HTTPMethod.PUT)
            logInfo(route.pattern);
    }
    logInfo("POST:");
    foreach (route; routes)
    {
        if (route.method == HTTPMethod.POST)
            logInfo(route.pattern);
    }
    logInfo("DELETE:");
    foreach (route; routes)
    {
        if (route.method == HTTPMethod.DELETE)
            logInfo(route.pattern);
    }
    logInfo("===============================");

    listenHTTP(settings, router);
    logInfo("Please open http://127.0.0.1:" ~ text(settings.port) ~ "/ in your browser.");
}
