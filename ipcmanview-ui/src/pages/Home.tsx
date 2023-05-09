import { Component, For, createResource, Switch, Match } from "solid-js";
import pb from "../pb";

type Station = {
  id: string;
  url: string;
  name: string;
};

const Home: Component = () => {
  const [stations] = createResource(() =>
    pb.collection("stations").getFullList<Station>()
  );

  return (
    <>
      <h1 class="text-3xl">Home Page</h1>
      <ul>
        <Switch>
          <Match when={stations.error}>
            <li>ERROR</li>
          </Match>
          <Match when={stations()}>
            <For each={stations()}>{(station) => <li>{station.name}</li>}</For>
          </Match>
        </Switch>
      </ul>
    </>
  );
};

export default Home;
