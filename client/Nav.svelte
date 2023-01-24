<!-- <a href="https://github.com/sachinbhutani/svelte-on-rust"> !-->
<script>
    import {user} from './store.js';
    import {Link} from 'yrv';
  import { loop_guard } from 'svelte/internal';

    let isActive = false;
    function toggleMenu(){
        isActive = !isActive;
    }

    function fetchi() {
        fetch("/turtle/movement", {
              method: 'POST',
              body: JSON.stringify({name: "Zeus"})
            })
    }
</script>

<style>
	:global([aria-current]) { font-weight: bold;}

    :global(a) {
        float: left;
    }
    
    .login {
        float: right;
    }

    :global(a) {
        display: block;
        color: white;
        text-align: center;
        padding: 13px 15px;
        text-decoration: none;
    }

    :global(a):hover {
        background-color: rgb(67, 185, 166);
        color: white;
    }
    
    nav {
        margin: 0;
        padding: 0;
        background-color:rgb(31, 39, 45);
    }

    .dropdown {
        position: relative;
        display: inline-block;
    }

    .dropdown:hover .dropbtn {
        background-color: rgb(67, 185, 166);
    }

    .dropbtn {
        background-color: rgb(31, 39, 45);
        color: white;
        padding: 16px;
        font-size: 16px;
        border: none;
        cursor: pointer;
    }

    /* The container <div> - needed to position the dropdown content */
    .dropdown {
        position: relative;
        display: inline-block;
    }

    /* Dropdown Content (Hidden by Default) */
    .dropdown-content {
        display: none;
        position: absolute;
        background-color: #f9f9f9;
        min-width: 162px;
        box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
        z-index: 1;
    }

    /* Links inside the dropdown */
    .dropdown-content a {
        margin-left: 6px;
        color: rgb(19, 18, 18);
        padding: 12px 16px;
        text-decoration: none;
        display: block;
    }

    /* Change color of dropdown links on hover */
    .dropdown-content a:hover {background-color: #f1f1f1}

    /* Show the dropdown menu on hover */
    .dropdown:hover .dropdown-content {
        display: block;
    }
</style>

<!-- svelte-ignore a11y-no-redundant-roles -->
<nav>
    <div>
        <div>
        <Link href="/"> Home </Link>
        <Link href="#about"> Svelte Route </Link>
            <div class="dropdown">
                <button class="dropbtn">
                Server Routes
                </button>
                <div class="dropdown-content">
                    <a href="/api">
                        String
                    </a>
                    <a href="/api/message">
                        JSON
                    </a>
                </div>
            </div>
        <Link href="#app/common?p1=test&p2=3"> Common Route </Link>
        {#if $user} <Link href="#secure"> Secure Route </Link>{/if}
        <Link href="#catch-it"> Catch All </Link>
        {#if $user} <Link href="#clock"> Clock </Link> {/if}
        <button id="turtles" onclick='window.location.href="turtles.html"'>Connect to turtles</button>
        {#if !$user}
            <div class="login">
                <Link href="#login">
                    Log in
                </Link>
            </div>
        {:else}
            <div class="login">
                <Link href="#logout">
                    Log Out
                </Link>
            </div>
        {/if}
        </div>
    </div>
</nav>
