import test_basic::*;
import glue::*;

#[test]
fn TestBit() {
    let conn = TestConnect();

    let testbit = Table("testbit", [
        ("did",        [Unique], SerialM),
        ("bitfield",   [Insert], BitM(3)),
    ]);

    // Insert
    Assure( testbit.DropTable(conn));
    Assure( testbit.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testbit.Insert([
        Bit([true, false, true])
    ]);

    assert q == "insert into testbit (bitfield) VALUES (B'101')";
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testbit";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == Bit([true, false, true]);    

    // This isn't serial, because there is no Ftype "serial"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
