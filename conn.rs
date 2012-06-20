// ------------------------------------------------------------------
class Conn {
    priv {
        let connstr: str;
        let conn: *PGconn;
    }

    new(connstr: str) {
        self.connstr = copy connstr;
        self.conn = str::as_c_str(connstr, pq::PQconnectdb);
    }

    // ------------------------------------------------------------------
    fn Status() -> str {
        ret alt pq::PQstatus(self.conn) {
          CONNECTION_OK { "CONNECTION_OK" }
          CONNECTION_BAD { "CONNECTION_BAD" }
          CONNECTION_STARTED { "CONNECTION_STARTED" }
          CONNECTION_MADE { "CONNECTION_MADE		" }
          CONNECTION_AWAITING_RESPONSE { "CONNECTION_AWAITING_RESPONSE" }
          CONNECTION_AUTH_OK { "CONNECTION_AUTH_OK" }
          CONNECTION_SETENV { "CONNECTION_SETENV" }
          CONNECTION_SSL_STARTUP { "CONNECTION_SSL_STARTUP" }
          CONNECTION_NEEDED  { "CONNECTION_NEEDED" }
        }
    }

    fn ResetStart() -> int {
        ret pq::PQresetStart(self.conn) as int;
    }

    fn LibVersion() -> int {
        pq::PQlibVersion() as int
    }

    fn ServerVersion() -> int {
        pq::PQserverVersion(self.conn) as int
    }

    fn ProtocolVersion() -> int {
        pq::PQprotocolVersion(self.conn) as int
    }

    unsafe fn Db() -> str {
        unsafe::from_c_str(pq::PQdb(self.conn))
    }

    unsafe fn User() -> str {
        unsafe::from_c_str(pq::PQuser(self.conn))
    }
    unsafe fn Pass() -> str {
        unsafe::from_c_str(pq::PQpass(self.conn))
    }
    unsafe fn Host() -> str {
        unsafe::from_c_str(pq::PQhost(self.conn))
    }
    unsafe fn Port() -> str {
        unsafe::from_c_str(pq::PQport(self.conn))
    }
    unsafe fn Tty() -> str {
        unsafe::from_c_str(pq::PQtty(self.conn))
    }
    unsafe fn Options() -> str {
        unsafe::from_c_str(pq::PQoptions(self.conn))
    }

    // ------------------------------------------------------------------
    fn Exec(query: str) -> Result {
        //fn PQexec(conn: *PGconn, query: *c_char) -> *PGresult;
        let r = str::as_c_str(query, bind pq::PQexec(self.conn, _));
        Result(r)
    }

}
