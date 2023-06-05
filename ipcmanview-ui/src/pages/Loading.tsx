import { Component } from "solid-js";
import { CenterLayout } from "~/ui/Layouts";

const Loading: Component = () => (
  <CenterLayout>
    <span class="loading loading-ring loading-lg m-auto" />
    <div class="text-center">IPCManView</div>
  </CenterLayout>
);

export default Loading;
