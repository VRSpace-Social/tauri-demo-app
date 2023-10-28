<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let username = "";
  let greetMsg = ""



  async function seeOnlinePlayers(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("see_online_players", { username })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={seeOnlinePlayers}>
    <!-- <input id="greet-input" placeholder="Enter a name..." bind:value={name} /> -->
    <input id="greet-1" placeholder="Enter a username..." bind:value={username} />
    <button type="submit">Greet</button>
  </form>
    <p>{greetMsg}</p>
</div>

