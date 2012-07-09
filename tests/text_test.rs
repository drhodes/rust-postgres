import test_basic::*;
import glue::*;

#[test]
fn TestText() {
    let conn = TestConnect();

    let testtext = Table("testtext", [
        ("did",        [Unique], SerialM),
        ("textfield",   [Insert], TextM),
    ]);

    // Insert
    Assure( testtext.DropTable(conn));
    Assure( testtext.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testtext.Insert([
        Text("What, in this moment, is lacking?")
    ]);

    assert q == "insert into testtext (textfield) VALUES ('What, in this moment, is lacking?')";
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testtext";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == Text("What, in this moment, is lacking?");


    // This isn't serial, because there is no Ftype "serial"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
