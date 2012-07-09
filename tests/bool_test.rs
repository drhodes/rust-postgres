import test_basic::*;
import glue::*;

#[test]
fn TestBool() {
    let conn = TestConnect();

    let testbool = Table("testbool", [
        ("did",        [Unique], SerialM),
        ("boolfld",      [Insert], BoolM),
    ]);

    // Insert
    Assure( testbool.DropTable(conn));
    Assure( testbool.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testbool.Insert([
        Bool(true)
    ]);
    
    assert q == "insert into testbool (boolfld) VALUES (TRUE)";
    log(error, q);
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testbool";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == Bool(true);

    // This isn't serial, because there is no Ftype "serial"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    //assert row[0] == Bool(true); 
}
