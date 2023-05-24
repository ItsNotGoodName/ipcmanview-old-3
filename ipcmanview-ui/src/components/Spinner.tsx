import { Component } from "solid-js";
import { RiSystemLoader4Fill } from "solid-icons/ri";

const Spinner: Component = () => {
  return <RiSystemLoader4Fill class="h-full w-6 animate-spin" />;
};

export default Spinner;
