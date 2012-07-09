import test_basic::*;
import glue::*;

#[test]
fn TestMacAddr() {
    let conn = TestConnect();

    let testmacaddr = Table("testmacaddr", [
        ("did",          [Unique], SerialM),
        ("macaddrfield", [Insert], MacAddrM),
    ]);

    // Insert
    Assure( testmacaddr.DropTable(conn));
    Assure( testmacaddr.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testmacaddr.Insert([
        MacAddr(0,0,0,0,34,234)
    ]);

    assert q == "insert into testmacaddr (macaddrfield) VALUES ('00:00:00:00:22:ea')";
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testmacaddr";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == MacAddr(0,0,0,0,34,234);

    // This isn't serial, because there is no Ftype "serial"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
