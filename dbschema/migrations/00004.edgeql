CREATE MIGRATION m1azc6mn4reqie2smm63zaglhed4cgxgof4nolzdofspj4zi4syoeq
    ONTO m1ib5plo3yqli47r2hae5kyacns5o7v4ceqdzkllrxitzpttzb2bjq
{
  CREATE TYPE default::Playlist {
      CREATE MULTI LINK songs: default::Song;
      CREATE REQUIRED PROPERTY name: std::str;
  };
  CREATE TYPE default::User {
      CREATE MULTI LINK playlists: default::Playlist;
      CREATE REQUIRED PROPERTY email: std::str;
      CREATE REQUIRED PROPERTY name: std::str;
      CREATE REQUIRED PROPERTY password: std::str;
      CREATE REQUIRED PROPERTY salt: std::str;
  };
  ALTER TYPE default::Playlist {
      CREATE MULTI LINK users := (.<playlists[IS default::User]);
  };
};
