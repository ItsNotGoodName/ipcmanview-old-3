import { Component } from "solid-js";
import { RiSystemLoader4Fill } from "solid-icons/ri";

const Spinner: Component = () => {
  return (
    <div class="overflow-hidden">
      <RiSystemLoader4Fill class="h-full w-6 animate-spin" />
    </div>
  );
};

export default Spinner;
