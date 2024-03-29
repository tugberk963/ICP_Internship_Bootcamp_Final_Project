<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";
    function onLogout() {
        backend.logout();
        window.location.href = "/";
    }
    let department_name,
        doctor_name,
        selected_date,
        selected_time = "";
    let departments,
        doctors,
        dates,
        times = [];

    let userData,
        providerData = {};
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

    async function getProviderData() {
        try {
            const providerDataString = await backend.get_provider_info(
                userData.identity,
            );
            providerData = JSON.parse(providerDataString[0]); // Parse JSON string to object
            console.log(providerData);
        } catch (error) {
            console.error("Error fetching user data:", error);
            alert("User data couldn't be fetched.");
        }
    }

    async function getDepartments() {
        try {
            departments = await backend.list_departments(userData.identity);
            console.log(departments);
        } catch (error) {
            console.error("Fetching departments failed.", error);
        }
    }

    async function addDepartment() {
        try {
            console.log(
                await backend.add_department(
                    userData.identity,
                    department_name,
                ),
            );
            alert("Department successfully added.");
        } catch (error) {
            console.error("Adding department failed.", error);
        }
    }

    async function getDoctors() {
        try {
            doctors = await backend.list_doctors(
                userData.identity,
                department_name,
            );
            console.log(doctors);
        } catch (error) {
            console.error("Fetching doctors failed.", error);
        }
    }

    async function addDoctor() {
        try {
            console.log(
                await backend.add_doctor(
                    userData.identity,
                    department_name,
                    doctor_name,
                ),
            );
            alert("Doctor successfully added.");
        } catch (error) {
            console.error("Adding doctor failed.", error);
        }
    }

    async function getDates() {
        try {
            dates = await backend.list_dates(
                userData.identity,
                department_name,
                doctor_name,
            );
            console.log(dates);
        } catch (error) {
            console.error("Fetching appointment dates failed.", error);
        }
    }

    async function addDate() {
        try {
            console.log(
                await backend.add_date(
                    userData.identity,
                    department_name,
                    doctor_name,
                    selected_date,
                ),
            );
            alert("Date successfully added.");
        } catch (error) {
            console.error("Adding date failed.", error);
        }
    }

    async function getTimes() {
        try {
            times = await backend.list_times(
                userData.identity,
                department_name,
                doctor_name,
                selected_date,
            );
            console.log(times);
        } catch (error) {
            console.error("Fetching appointment times failed.", error);
        }
    }

    async function addTime() {
        try {
            console.log(
                await backend.add_time(
                    userData.identity,
                    department_name,
                    doctor_name,
                    selected_date,
                    selected_time,
                ),
            );
            alert("Time successfully added.");
        } catch (error) {
            console.error("Adding time failed.", error);
        }
    }

    onMount(async () => {
        await get_user_data();
        await getProviderData();
    });
</script>

<main>
    <navbar>
        <div class="logo">
            <img src="../logo.png" alt="logo" />
        </div>
    </navbar>
    <div class="generalContainer">
        <div class="sections">
            <h2>Provider Settings</h2>
            <a href="/home">Home</a>
            <a href="/provider_management">Provider Management</a>
            <a href="/search_patients">Search Patients</a>
            <a href="/manage_patients">Manage Patients</a>
            <a href="/provider_profile">Provider Profile</a>
        </div>
        <!-- Middle Container for Detailed Actions -->
        <div class="container">
            <div class="section">
                <h2>Departments</h2>
                {#if departments}
                {#each departments as department}
                    <div class="department-option">{department}</div>
                {/each}
             {/if}
                <button on:click={getDepartments}>List Departments</button>

                <input
                    type="text"
                    placeholder="Enter department name"
                    bind:value={department_name}
                />
                <button on:click={addDepartment}>Add Department</button>
            </div>
            <div class="section">
                <h2>Doctors</h2>
                <select
                    name="department-name"
                    id="department"
                    bind:value={department_name}
                >
                    <option value="">Select Department</option>
                    {#if departments}
                        {#each departments as department}
                            <option value={department}>{department}</option>
                        {/each}
                    {/if}
                </select>
                {#if doctors}
                    {#each doctors as doctor}
                        <div class="doctor-option">{doctor}</div>
                    {/each}
                {/if}
                <button on:click={getDoctors}>List Doctors</button>
                <input
                    type="text"
                    placeholder="Enter doctor name"
                    bind:value={doctor_name}
                />
                <button on:click={addDoctor}>Add Doctor</button>
            </div>
            <div class="section">
                <h2>Appointment Dates</h2>
                <select name="doctor-name" id="doctor" bind:value={doctor_name}>
                    <option value="">Select Doctor</option>
                    {#if doctors}
                        {#each doctors as doctor}
                            <option value={doctor}>{doctor}</option>
                        {/each}
                    {/if}
                </select>
                {#if dates}
                    {#each dates as date}
                        <div class="date-option">{date}</div>
                    {/each}
                {/if}
                <button on:click={getDates}>List Appointment Dates</button>

                <input type="date" bind:value={selected_date} />
                <button on:click={addDate}>Add Date</button>
            </div>
            <div class="section">
                <h2>Appointment Times</h2>
                <select
                    name="selected-date"
                    id="date"
                    bind:value={selected_date}
                >
                    <option value="">Select Date</option>
                    {#if dates}
                        {#each dates as date}
                            <option value={date}>{date}</option>
                        {/each}
                    {/if}
                </select>
                {#if times}
                    {#each times as time}
                        <div class="time-option">{time}</div>
                    {/each}
                {/if}
                <button on:click={getTimes}>List Appointment Times</button>

                <input type="time" bind:value={selected_time} />
                <button on:click={addTime}>Add Time</button>
            </div>
        </div>
        <!-- Profile Section -->
        <div class="personalInformation">
            <div class="personPic">
                <img src="../../person.png" alt="personPic" />
            </div>
            <div class="personalData">
                {#if userData && providerData}
                    <p class="dfinityID">Dfinity ID: {userData.identity}</p>
                    <p class="username">Username: {userData.username}</p>
                    {#if !providerData.provider_name}
                        <p class="name">Provider Name: Loading..</p>
                    {:else}
                        <p class="name">
                            Provider Name: {providerData.provider_name}
                        </p>
                    {/if}
                    {#if !providerData.provider_location}
                        <p class="location">
                            Provider Location: Please add your location.
                        </p>
                    {:else}
                        <p class="location">
                            Provider Location: {providerData.provider_location}
                        </p>
                    {/if}
                {:else}
                    <p class="dfinityID">Dfinity ID: N/A</p>
                    <p class="username">Username: N/A</p>
                    <p class="name">Provider Name: N/A</p>
                    <p class="surname">Provider Location: N/A</p>
                {/if}
            </div>
            <br />
            <button on:click|preventDefault={onLogout}
                >Click for logout..</button
            >
        </div>
    </div>
</main>

<style>
    main {
        font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
        display: flex;
        flex-direction: column;
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

    .personalInformation {
        width: 150px;
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-left: 20px;
    }

    .personPic img {
        width: 100px;
        height: 100px;
        border-radius: 50%;
        margin-bottom: 20px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    a {
        color: #333;
        text-decoration: none;
        font-size: 18px;
        margin-bottom: 10px;
        display: block;
        transition: color 0.3s;
    }

    a:hover {
        color: #007bff;
    }

    main {
        font-size: 16px;
        display: flex;
        flex-direction: column;
    }

    .generalContainer {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding: 20px;
    }

    button {
        background-color: #007bff;
        color: #fff;
        border: none;
        border-radius: 5px;
        padding: 10px 20px;
        cursor: pointer;
        font-size: 16px;
        transition: background-color 0.3s;
    }

    button:hover {
        background-color: #0056b3;
    }

    .section {
        flex-basis: 45%;
        margin-bottom: 20px;
    }

    h2 {
        margin-bottom: 10px;
    }

    button {
        margin-bottom: 10px;
        padding: 8px 12px;
        border: none;
        border-radius: 4px;
        background-color: #007bff;
        color: #fff;
        cursor: pointer;
    }

    input[type="text"],
    input[type="date"],
    input[type="time"],
    select {
        width: calc(100% - 12px);
        padding: 8px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .date-option,
    .time-option,
    .department-option,
    .doctor-option {
        margin-bottom: 5px;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
        background-color: #f9f9f9;
    }

    .logo img {
        width: 100px;
        height: auto;
    }
</style>
