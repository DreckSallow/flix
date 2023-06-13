import { ref } from "vue";

export interface IWorkspaceData {
  name: string;
}

export type TWorkspaceProvide = IWorkspaceData | null;

export const useWorkspaceProvider = () => {
  const workspaceData = ref<TWorkspaceProvide>(null);

  return {
    workspaceData,
    setData(d: TWorkspaceProvide) {
      workspaceData.value = d;
    },
  };
};

export const workspaceKeyProv = "current-workspace";
