import test_basic::*;
import glue::*;

#[test]
fn TestVarBit() {
    let conn = TestConnect();
    
    let testvarbit = Table("testvarbit", [
        ("did",        [Unique], SerialM),
        ("varbitfield",   [Insert], VarBitM(3)),
    ]);

    // Insert
    Assure( testvarbit.DropTable(conn));
    Assure( testvarbit.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testvarbit.Insert([
        VarBit([true, false, true])
    ]);

    assert q == "insert into testvarbit (varbitfield) VALUES (B'101')";
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testvarbit";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == VarBit([true, false, true]);    

    assert row[0] == Int32(1); 
}
