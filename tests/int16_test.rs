import test_basic::*;
import glue::*;

#[test]
fn TestInt16() {
    let conn = TestConnect();

    let testint16 = Table("testint16", [
        ("did",        [Unique], SerialM),
        ("int16field", [Insert], Int16M),
    ]);

    // Insert
    Assure( testint16.DropTable(conn));
    Assure( testint16.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testint16.Insert([
        Int16(1<<12) // todo (1<<32)-1 should work. Why does it overflow postgres?
    ]);

    assert q == "insert into testint16 (int16field) VALUES (4096)";    
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testint16";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    


    alt row[1] {
      Int16(n) {        
        assert n == 1 << 12;
      }
      _ { 
        log(error, "Int16 Error--------------------------------------------");
        log(error, row[1]);
        fail("pq: wrong Int16 value from db");
      }
    }

    // This isn't Serial(1) because there is no Ftype "SERIAL"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
