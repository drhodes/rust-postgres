Bindings for postgres (libpq 9.1.1)
=============
These aren't ready yet :)


Testing
-------

To test the bindings you'll need to get into postgres as a superuser and create a user 

Here's what my session looked like

<pre>
~$ psql
psql (9.1.1)
Type "help" for help.

postgres=# <b>CREATE USER rustuser WITH PASSWORD 'rustpass';</b>
<i>CREATE ROLE</i>

postgres=# <b>CREATE DATABASE rust_test_db;</b>
<i>CREATE DATABASE</i>

</pre>

Example Usage (totally up in the air, still learning rust. If you see something
obviously brain dead and you care enough, please make an issue)

<pre>
fn AlgebraTest() {
    let conn = TestConnect();

    let person = Table("person", [
        ("did",    [Unique], SerialM),
        ("name",   [Insert], VarCharM(255))
    ]);

    let movie = Table("movie_algebra", [
        ("did",      [Unique],                                 SerialM),
        ("title",    [Insert],                                 VarCharM(255)),
        ("year",     [Insert],                                 Int32M),
        ("director", [Insert, ForeignKey(copy person, "did")], Int32M )                                                
    ]);

    // Drop some tables
    Assure( conn.Exec( DropTable( person)));
    Assure( conn.Exec( DropTable( movie)));

    // Create some tables
    Assure( conn.Exec( CreateTable( person)));
    Assure( conn.Exec( CreateTable( movie)));

    // Test the fancy insert.
    let q1 = person.Insert( [VarChar("lucas")] );
    assert q1 == "insert into person (name) VALUES ('lucas')";
    Assure(conn.Exec(q1));

    // no fancy select yet.                                                                                            
    // Get the person.did of lucas to test foreign key insert                                                          
                                                                                                                       
    unsafe {
        let res = Assure(conn.Exec("select * from person where name = 'lucas'"));
        let lucas_did = copy GetRow(res, 0)[0];

        let q2 =  movie.Insert( [VarChar("starwars"), Int32(1977), lucas_did]);
        assert q2 ==
            "insert into movie_algebra (title,year,director) VALUES ('starwars',1977,1)";
        Assure(conn.Exec(q2));
    }
</pre>






Preliminary Results
-------------------









If postgres is setup correctly and rustc can find your libpq then you might be 
able to get the following output

<pre>

$ rustc --bin pq.rc --test && ./pq

running 5 tests
NOTICE:  table "movie2" does not exist, skipping
NOTICE:  table "movie4" does not exist, skipping
NOTICE:  table "movie3" does not exist, skipping
NOTICE:  table "movie1" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie2_did_seq" for serial column "movie2.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie4_did_seq" for serial column "movie4.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie3_did_seq" for serial column "movie3.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie1_did_seq" for serial column "movie1.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie2_did_key" for table "movie2"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie3_did_key" for table "movie3"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie4_did_key" for table "movie4"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie1_did_key" for table "movie1"
test glue::GetAllRowTest ... ok
test glue::UseCase2 ... ok
test ResultTest ... ok
test glue::UseCase1 ... ok
NOTICE:  table "movie" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie_did_seq" for serial column "movie.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_did_key" for table "movie"
test glue::GetRowTest ... ok

result: ok. 5 passed; 0 failed; 0 ignored

</pre>
