import { useParams } from "@solidjs/router";
import { Component, For } from "solid-js";
import {
  useCameras,
  useCamerasTotal,
  useScansCompleted,
  useScansPending,
} from "../hooks";
import { usePb } from "../pb";

const StationsShow: Component = () => {
  const pb = usePb();

  const { id } = useParams<{ id: string }>();
  const stationId = () => id;

  const cameras = useCameras(pb, stationId);
  const camerasTotal = useCamerasTotal(pb, stationId);
  const scansPending = useScansPending(pb, stationId);
  const scansCompleted = useScansCompleted(pb, stationId);

  return (
    <div class="flex flex-col gap-2">
      <Thing title="Cameras" data={cameras.data} />

      <For each={cameras.data || []}>
        {(camera) => {
          return (
            <div class="ml-2">
              <Thing title={"Camera " + camera.id} data={""} />
            </div>
          );
        }}
      </For>

      <Thing title="Cameras Total" data={camerasTotal.data} />
      <Thing title="Pending Scans" data={scansPending.data} />
      <Thing title="Completed Scans" data={scansCompleted.data} />
    </div>
  );
};

const Thing: Component<{ title: string; data?: any }> = (props) => (
  <div class="border p-2" id={props.title}>
    <a class="text-xl font-bold" href={"#" + props.title}>
      {props.title}
    </a>
    <pre>{JSON.stringify(props.data, null, 2)}</pre>
  </div>
);

export default StationsShow;
