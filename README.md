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

 ...
 result: ok. 22 passed; 0 failed; 0 ignored

</pre>
