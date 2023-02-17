import { reactive } from "vue";

export const store = reactive({
  API: "https://localhost:8080/",
  playlist: [],
  currentTrack: null,

  add_track: async function (id: number) {
    let track = await (await fetch(this.API + "tracks/" + id)).json();
    // @ts-ignore
    if (this.currentTrack != null) {
      // @ts-ignore
      this.playlist.push(track[0]);
    } else {
      this.currentTrack = track[0];
    }
  },

  next_track: async function () {
    if (this.playlist.length > 0) {
      // @ts-ignore
      this.currentTrack = this.playlist.shift();
    }
  },

  add_album: function (album) {
    for (let track of album.track_ids) {
      this.add_track(track);
    }

    console.log(this.playlist);
    console.log(this.currentTrack);
  },
});
