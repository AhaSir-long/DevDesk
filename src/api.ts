import { invoke } from "@tauri-apps/api/core";
import type { ServiceInstance, ServiceInstanceDraft } from "./types";

export function listInstances(): Promise<ServiceInstance[]> {
  return invoke<ServiceInstance[]>("list_instances");
}

export function saveInstance(instance: ServiceInstanceDraft): Promise<void> {
  return invoke<void>("save_instance", { instance });
}
