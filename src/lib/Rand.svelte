<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

    let rand_int: string;

    async function gen_rand_int() {
        rand_int = await invoke('random', {});
    }
</script>

<div>
    <button on:click={gen_rand_int}>Generate random integer</button>
    <p>{rand_int}</p>
</div>