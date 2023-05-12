import { RiSystemLoader4Fill } from "solid-icons/ri";
import { Component, For, createResource, Switch, Match, Show } from "solid-js";
import pb, { StationRecord } from "../pb";

const Home: Component = () => {
  return (
    <div class="flex">
      <StationListCard />
    </div>
  );
};

const StationListCard: Component = () => {
  const [stations] = createResource(() =>
    pb.collection("stations").getFullList<StationRecord>()
  );

  return (
    <div class="flex flex-1 flex-col rounded p-2 shadow shadow-ship-300 sm:max-w-md">
      <h1 class="mx-auto text-xl">Stations</h1>
      <div class="rounded p-2">
        <Switch>
          <Match when={stations.error}>
            <div class="text-danger">{stations.error.message}</div>
          </Match>
          <Match when={stations.loading}>
            <RiSystemLoader4Fill class="mx-auto h-6 w-6 animate-spin" />
          </Match>
          <Match when={stations()}>
            <Show
              when={stations()!.length > 0}
              fallback={<div>No stations.</div>}
            >
              <table class="w-full">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>URL</th>
                  </tr>
                </thead>
                <tbody>
                  <For each={stations()}>
                    {(station) => (
                      <tr>
                        <td>{station.name}</td>
                        <td>
                          <a href={station.url}>{station.url}</a>
                        </td>
                      </tr>
                    )}
                  </For>
                </tbody>
              </table>
            </Show>
          </Match>
        </Switch>
      </div>
    </div>
  );
};

export default Home;
