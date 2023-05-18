module default {
    type Artist {
        required name: str;

        multi albums: Album;
        multi songs: Song;
    }

    type Album {
        required title: str;
        multi songs: Song;

        required total_tracks: int64;

        multi link artists := .<albums[is Artist];
    }

    type Song {
        required title: str;
        required length: duration;

        tracknumber: int64;
        release_year: int64;

        genre: str;
        mood: str;

        multi link artists := .<songs[is Artist];
        multi link albums := .<songs[is Album];
    }

    type User {
        required name: str;
        required email: str;
        required password: str;
        required salt: str;

        multi playlists: Playlist;
    }

    type Playlist {
        required name: str;

        multi songs: Song;
        multi link users := .<playlists[is User];
    }

    type Client {
        required secret: str;
    }
};