import test_basic::*;
import glue::*;

#[test]
fn TestUuid() {
    let conn = TestConnect();

    let testuuid = Table("test_uuid", [
        ("did",           [Unique], SerialM),
        ("uuid_field",    [Insert], UuidM),
    ]);

    // Insert
    Assure( testuuid.DropTable(conn));
    Assure( testuuid.CreateTable(conn));

    let tval = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]/16;

    // exclude the "did" field because it doesn't have the Insert attribute
    let q = testuuid.Insert([
        Uuid(tval)
    ]);
    
    //assert q == "insert into testuuid (uuidfld) VALUES (00000000-0000-0000-0000-000000000000)";
    Assure(conn.Exec(q));

    // Select
    let s = "select * from test_uuid";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);  
    assert row[1] == Uuid(tval);
}
