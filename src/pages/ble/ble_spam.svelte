<script>
    import {
      Page,
      Navbar,
      NavTitle,
      Button,
      Block,
    } from "framework7-svelte";
    import { onMount } from "svelte";
  
    export let f7router;
    export let f7route;
    
    let _ip = "";
    let ws;
    
    onMount(() => {
      _ip = localStorage.getItem("ip");
      ws = new WebSocket("ws://" + _ip + '/' + f7route.params.type);
    });

    function stop_attack() {
      ws.send("stop");
      f7router.back();
    }
    console.log(f7route);
  </script>
  
  <Page name="home">
    <!-- Top Navbar -->
    <Navbar sliding={false} backLink="Back">
      <NavTitle sliding>CapibaraZero</NavTitle>
    </Navbar>
    
    <Block>
        <p>{f7route.params.type} attack in progress...</p>
    </Block>

    <Button on:click={stop_attack}>Stop attack</Button>
  </Page>
  