import test_basic::*;
import glue::*;

#[test]
fn TestInt64() {
    let conn = TestConnect();

    let testint64 = Table("testint64", [
        ("did",        [Unique], SerialM),
        ("int64field", [Insert], Int64M),
    ]);

    // Insert
    Assure( testint64.DropTable(conn));
    Assure( testint64.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testint64.Insert([
        Int64(1 << 30) // todo (1<<32)-1 should work. Why does it overflow postgres?
    ]);
    log(error, q);
    assert q == "insert into testint64 (int64field) VALUES (1073741824)";
    Assure(conn.Exec(q));
    // Select
    let s = "select * from testint64";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Int64(n) {        
        assert n == 1 << 30;
      }
      _ { 
        log(error, row[1]);
        fail("wrong value from db");
      }
    }

    // This isn't Serial(1) because there is no Ftype "SERIAL"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
