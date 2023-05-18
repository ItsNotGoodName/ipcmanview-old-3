import { RiSystemLoader4Fill } from "solid-icons/ri";
import { Component, For, createResource, Switch, Match, Show } from "solid-js";
import { Args } from "storybook-solidjs";
import Card from "../components/Card";
import pb, { StationRecord, stationUrl } from "../pb";

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
    <div class="flex w-full flex-wrap gap-4">
      <Card title="Stations" class="sm:max-w-md">
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
                    <th>Url</th>
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
      </Card>
      <Show when={!stations.error && !stations.loading && stations()}>
        <For each={stations()}>{(s) => <StationCard station={s} />}</For>
      </Show>
    </div>
  );
};

type Camera = {
  id: number;
  ip: string;
  username: string;
};

const StationCard: Component<{ station: StationRecord }> = (props) => {
  const [cameras] = createResource(() =>
    pb.send<Array<Camera>>(stationUrl(props.station.id, "/cameras"), {})
  );

  return (
    <Card title={"Station - " + props.station.name} class="sm:max-w-md">
      <Switch>
        <Match when={cameras.error}>
          <div class="text-danger">{cameras.error.message}</div>
        </Match>
        <Match when={cameras.loading}>
          <RiSystemLoader4Fill class="mx-auto h-6 w-6 animate-spin" />
        </Match>
        <Match when={cameras()}>
          <Show when={cameras()!.length > 0} fallback={<div>No cameras.</div>}>
            <table class="w-full">
              <thead>
                <tr>
                  <th>Id</th>
                  <th>Ip</th>
                  <th>username</th>
                </tr>
              </thead>
              <tbody>
                <For each={cameras()}>
                  {(station) => (
                    <tr>
                      <td>{station.id}</td>
                      <td>
                        <a class="text-link" href={"http://" + station.ip}>
                          {station.ip}
                        </a>
                      </td>
                      <td>{station.username}</td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </Show>
        </Match>
      </Switch>
    </Card>
  );
};

export default Home;
