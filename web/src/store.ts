import { reactive } from "vue";

export const store = reactive({
  API: "https://localhost:8080/",
  playlist: [],
  currentTrack: null,
  playing: false,
  albums: [],
  artists: [],

  add_track: async function (id: number) {
    let track = await (await fetch(this.API + "tracks/" + id)).json();
    // @ts-ignore
    if (this.currentTrack != null) {
      // @ts-ignore
      this.playlist.push(track[0]);
    } else {
      this.currentTrack = track[0];
      this.playing = true;
    }
  },

  next_track: async function () {
    if (this.playlist.length > 0) {
      // @ts-ignore
      this.currentTrack = this.playlist.shift();
    } else {
      this.currentTrack = null;
    }
  },

  add_album: async function (album) {
    for (let track of album.track_ids) {
      await this.add_track(track);
    }

    console.log(this.playlist);
    console.log(this.currentTrack);
  },

  togglePlaying: function () {
    this.playing = !this.playing;
  },

  getAlbums: async function () {
    this.albums = await (await fetch(this.API + "albums")).json();
  },

  getArtists: async function () {
    this.artists = await (await fetch(this.API + "artists")).json();
  },
});
