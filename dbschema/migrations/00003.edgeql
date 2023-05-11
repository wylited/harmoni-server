CREATE MIGRATION m1ib5plo3yqli47r2hae5kyacns5o7v4ceqdzkllrxitzpttzb2bjq
    ONTO m1hbswagt2d3rrb3v3b6l3u3ttomo4vvql6uqqxw2g56lhsjhfquja
{
  ALTER TYPE default::Album {
      DROP LINK artists;
      DROP PROPERTY title;
  };
  ALTER TYPE default::Artist {
      DROP LINK albums;
  };
  DROP TYPE default::Album;
  CREATE TYPE default::Album {
      CREATE MULTI LINK songs: default::Song;
      CREATE REQUIRED PROPERTY title: std::str;
      CREATE REQUIRED PROPERTY total_tracks: std::int64;
  };
  ALTER TYPE default::Artist {
      CREATE MULTI LINK albums: default::Album;
      CREATE MULTI LINK songs: default::Song;
  };
  ALTER TYPE default::Album {
      CREATE MULTI LINK artists := (.<albums[IS default::Artist]);
  };
  ALTER TYPE default::Song {
      CREATE MULTI LINK albums := (.<songs[IS default::Album]);
      CREATE MULTI LINK artists := (.<songs[IS default::Artist]);
  };
};
