import clsx from "clsx";
import {
  RiSystemDeleteBinFill,
  RiSystemRefreshFill,
  RiSystemSearchFill,
} from "solid-icons/ri";
import { Component } from "solid-js";
import Spinner from "./Spinner";

type ActionButtonsProps = {
  class?: string;
  isDeleteLoading?: boolean;
  onDelete?: () => void;
  isRefreshLoading?: boolean;
  onRefresh?: () => void;
  isScanLoading?: boolean;
  onScan?: () => void;
};

const ActionButtons: Component<ActionButtonsProps> = (props) => {
  return (
    <div class={clsx("flex gap-2", props.class)}>
      <button
        class="rounded bg-danger p-2 text-ship-50 hover:bg-danger-200"
        title="Delete"
        disabled={props.isDeleteLoading}
        onClick={props.onDelete}
      >
        {props.isDeleteLoading ? (
          <Spinner />
        ) : (
          <RiSystemDeleteBinFill class="h-6 w-6" />
        )}
      </button>
      <button
        class="rounded bg-ship-500 p-2 text-ship-50 hover:bg-ship-600"
        title="Refresh"
        disabled={props.isRefreshLoading}
        onClick={props.onRefresh}
      >
        <RiSystemRefreshFill
          class="h-full w-6"
          classList={{ "animate-spin": props.isRefreshLoading }}
        />
      </button>
      <button
        class="rounded bg-ship-500 p-2 text-ship-50 hover:bg-ship-600"
        title="Full Scan"
        disabled={props.isScanLoading}
        onClick={props.onScan}
      >
        {props.isScanLoading ? (
          <Spinner />
        ) : (
          <RiSystemSearchFill class="h-6 w-6" />
        )}
      </button>
    </div>
  );
};

export default ActionButtons;
