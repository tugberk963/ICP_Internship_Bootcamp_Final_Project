<script>
    import "../../index.scss";
    import { onMount } from "svelte";
    import User from "./User.svelte";
    import { backend } from "$lib/canisters";

    let isProvider;
    let userData = {};

    async function user_is_provider() {
        try {
            isProvider = await backend.is_provider();
            console.log(isProvider);
        } catch (error) {
            console.error("Error while checking user is provider: ", error);
        }
    }

    async function get_user_data() {
        try {
            const userDataString = await backend.get_current_user();
            userData = JSON.parse(userDataString[0]); // Parse JSON string to object
            console.log(userData);
        } catch (error) {
            console.error("Error fetching user data:", error);
            alert("User data couldn't be fetched.");
        }
    }

    async function set_provider(identity) {
        try {
            await backend.set_provider(identity);
            await user_is_provider();
            alert("Provider successfully added.");  
        } catch (error) {
            console.error("Error setting provider:", error);
        }
    }

    async function remove_provider(identity) {
        try {
            await backend.remove_provider(identity);
            await user_is_provider();
            alert("Provider successfully removed.");  
        } catch (error) {
            console.error("Error removing provider:", error);
        }
    }

    function goToProviderSettings(){
        window.location.href = "/provider_management";
    }

    onMount(async () => {
        await user_is_provider();
        await get_user_data();
    });
</script>

<main>
    <navbar>
        <div class="logo">
            <img src="../logo.png" alt="logo">
        </div>
        <div class="settings">
            {#if isProvider}
            <button on:click={goToProviderSettings}>Provider Settings</button>
            {/if}
            <button on:click={() => set_provider(userData.identity)}>Set Provider </button>
            <button on:click={() => remove_provider(userData.identity)}>Remove Provider</button>
        </div>
    </navbar>
    <User/>
</main>

<style>
    main {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }

    navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        background-color: #fcfcfc;
        color: #fff;
        box-shadow: 0px 2px 5px rgba(0, 0, 0, 0.1);
    }

    .logo img {
        width: 100px;
        height: auto;
    }

    .settings button {
        background-color: transparent;
        border: none;
        color: #a63d3d;
        font-size: 16px;
        cursor: pointer;
        transition: color 0.3s;
    }

    .settings button:hover {
        color: #f0f0f0;
    }
</style>
