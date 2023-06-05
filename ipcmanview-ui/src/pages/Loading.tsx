import { Component } from "solid-js";
import { CenterLayout } from "~/ui/Layouts";

const Loading: Component = () => (
  <CenterLayout>
    <div class="text-center text-xl font-bold">IPCManView</div>
    <span class="loading loading-ring loading-lg m-auto" />
  </CenterLayout>
);

export default Loading;
