<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let name: string;
	let greetMsg: string;

    async function greet() {
        greetMsg = await invoke('greet', { name })
    }
</script>

<div>
    <input id="greet-input" bind:value={name} placeholder="Enter a name..."/>
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
</div>