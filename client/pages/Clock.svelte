<script>

    import { readable } from 'svelte/store'

    const time = readable(null, function start(set) {
        const interval = setInterval(() => {
            set(new Date());
        }, 1000);

        return function stop() {
            clearInterval(interval)
        }
    });

    const formatter = new Intl.DateTimeFormat('en', {
            hour12: false,
            hour: 'numeric',
            minute: '2-digit',
            second: '2-digit'
        });
</script>

<h1>Current time: {formatter.format($time)}</h1>

<style>
    h1 {
        text-align: center;
        position: static;
        margin-top: 100px;
        margin-bottom: 100px;
        font-size: 40px;
    }
</style>