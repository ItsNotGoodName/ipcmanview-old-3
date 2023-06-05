import { Component } from "solid-js";

const Spinner: Component = () => {
  return (
    <div class="flex overflow-hidden">
      <span class="loading loading-spinner my-auto" />
    </div>
  );
};

export default Spinner;
