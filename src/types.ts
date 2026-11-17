export interface ServiceInstanceDraft {
  name: string;
  kind: "windowsService";
  serviceName: string;
  expectedPort: number | null;
  configPath: string | null;
  logPath: string | null;
}

export interface ServiceInstance extends ServiceInstanceDraft {
  id: string;
  startCommand: string | null;
  workDirectory: string | null;
  stopCommand: string | null;
}
