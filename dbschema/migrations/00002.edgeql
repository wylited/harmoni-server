CREATE MIGRATION m1hbswagt2d3rrb3v3b6l3u3ttomo4vvql6uqqxw2g56lhsjhfquja
    ONTO m1pxpvelvlavsjkzrwtcfw7norp3frabshswjzsqcgxmock3m5lk5q
{
  ALTER TYPE default::Movie {
      DROP LINK actors;
  };
  ALTER TYPE default::Movie RENAME TO default::Album;
  ALTER TYPE default::Person RENAME TO default::Artist;
  ALTER TYPE default::Artist {
      CREATE MULTI LINK albums: default::Album;
  };
  ALTER TYPE default::Album {
      CREATE MULTI LINK artists := (.<albums[IS default::Artist]);
  };
  CREATE TYPE default::Song {
      CREATE PROPERTY genre: std::str;
      CREATE REQUIRED PROPERTY length: std::duration;
      CREATE PROPERTY mood: std::str;
      CREATE PROPERTY release_year: std::int64;
      CREATE REQUIRED PROPERTY title: std::str;
      CREATE PROPERTY tracknumber: std::int64;
  };
};
