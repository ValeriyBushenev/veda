/**
 * LMDB реализация хранилища
 */
module veda.storage.lmdb.lmdb_storage;

import veda.common.logger, properd;
import veda.common.type, veda.storage.common, veda.storage.storage;
import veda.storage.lmdb.lmdb_driver;

const string individuals_db_path = "./data/lmdb-individuals";
const string tickets_db_path     = "./data/lmdb-tickets";

public class LmdbStorage : Storage
{
    private KeyValueDB tickets_storage_r;
    private KeyValueDB inividuals_storage_r;

    this(string _name, Logger _log)
    {
        log  = _log;
        name = _name;
    }

    ~this()
    {
        if (tickets_storage_r !is null)
            tickets_storage_r.close();

        if (inividuals_storage_r !is null)
            inividuals_storage_r.close();
    }

    override KeyValueDB get_tickets_storage_r(){
        if (tickets_storage_r is null)
            tickets_storage_r = new LmdbDriver(tickets_db_path, DBMode.R, name ~ ":tickets", log);

        return tickets_storage_r;
    }

    override KeyValueDB get_inividuals_storage_r(){
        if (inividuals_storage_r is null)
            inividuals_storage_r = new LmdbDriver(individuals_db_path, DBMode.R, name ~ ":inividuals", log);

        return inividuals_storage_r;
    }

    override long count_individuals(){
        return get_inividuals_storage_r().count_entries();
    }
}



