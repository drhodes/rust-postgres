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

Here's one of the tests
-----------------------
<i>totally up in the air, I'm still learning rust. If you see something obviously brain dead please make an issue.</i>

<pre>
#[test]
fn AlgebraTest() {
    // grab a connection
    let conn = TestConnect();

    // define a person 
    let person = Table("person", [
        ("did",    [Unique], SerialM),
        ("name",   [Insert], VarCharM(255))
    ]);

    // define a movie
    let movie = Table("movie_algebra", [
        ("did",      [Unique],                                 SerialM),
        ("title",    [Insert],                                 VarCharM(255)),
        ("year",     [Insert],                                 Int32M),
        ("director", [Insert, ForeignKey(copy person, "did")], Int32M ) 
    ]);

    // Drop some tables, just in case they're there.
    Assure(conn.Exec(DropTable(person)));
    Assure(conn.Exec(DropTable(movie)));

    // Create some tables
    Assure(conn.Exec(CreateTable(person)));
    Assure(conn.Exec(CreateTable(movie)));

    // Test the fancy insert.
    let q1 = person.Insert( [VarChar("lucas")] );

    assert q1 == "insert into person (name) VALUES ('lucas')";
    Assure(conn.Exec(q1));
                                                                                                                     
    unsafe {
        // no fancy select yet. 
        // Grab the person.did of lucas
        let res = Assure(conn.Exec("select * from person where name = 'lucas'"));
        let lucas_did = copy GetRow(res, 0)[0]; // this index munging goes away

		// 
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

$ rustc --bin pq.rc --test -W no-old-vecs && ./pq

running 8 tests
NOTICE:  table "movie1" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie1_did_seq" for serial column "movie1.did"
NOTICE:  table "movie3" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie3_did_seq" for serial column "movie3.did"
NOTICE:  drop cascades to constraint movie_alg_director_fkey on table movie_alg
NOTICE:  CREATE TABLE will create implicit sequence "testbit_did_seq" for serial column "testbit.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "testbit_did_key" for table "testbit"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie3_did_key" for table "movie3"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie1_did_key" for table "movie1"
NOTICE:  CREATE TABLE will create implicit sequence "person_did_seq" for serial column "person.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "person_did_key" for table "person"
NOTICE:  CREATE TABLE will create implicit sequence "movie_alg_did_seq" for serial column "movie_alg.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_alg_did_key" for table "movie_alg"
NOTICE:  table "movie4" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie4_did_seq" for serial column "movie4.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie4_did_key" for table "movie4"
NOTICE:  table "movie" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie_did_seq" for serial column "movie.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_did_key" for table "movie"
test bit_test::TestBit ... ok
NOTICE:  CREATE TABLE will create implicit sequence "testbool_did_seq" for serial column "testbool.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "testbool_did_key" for table "testbool"
NOTICE:  table "movie2" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie2_did_seq" for serial column "movie2.did"
rust: ~"insert into testbool (boolfld) VALUES (TRUE)"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie2_did_key" for table "movie2"
test facemelt1_test::FaceMelt1 ... ok
test test_basic::ResultTest ... ok
test algebra_movie_test::MovieTest ... ok
test facemelt2_test::UseCase2 ... ok
test get_row_test::GetRowTest ... ok
test bool_test::TestBool ... ok
test get_all_row_test::GetAllRowTest ... ok

result: ok. 8 passed; 0 failed; 0 ignored

</pre>
