<script setup lang="ts">
import { ref } from "vue";
import type { ServiceInstanceDraft } from "../types";

defineProps<{ disabled?: boolean; saveError?: string }>();
const emit = defineEmits<{ save: [instance: ServiceInstanceDraft]; cancel: [] }>();
const name = ref("");
const serviceName = ref("");
const port = ref("");
const error = ref("");

function submit() {
  const cleanName = name.value.trim();
  const cleanServiceName = serviceName.value.trim();
  if (!cleanName) {
    error.value = "请输入实例名称";
    return;
  }
  if (!cleanServiceName) {
    error.value = "请输入 Windows 服务名";
    return;
  }
  const expectedPort = port.value === "" ? null : Number(port.value);
  if (expectedPort !== null && (!Number.isInteger(expectedPort) || expectedPort < 1 || expectedPort > 65535)) {
    error.value = "端口必须在 1 到 65535 之间";
    return;
  }
  error.value = "";
  emit("save", {
    name: cleanName,
    kind: "windowsService",
    serviceName: cleanServiceName,
    expectedPort,
    configPath: null,
    logPath: null,
  });
}
</script>

<template>
  <form class="instance-form" novalidate @submit.prevent="submit">
    <label>实例名称<input name="name" v-model="name" placeholder="例如 MySQL" /></label>
    <label>Windows 服务名<input name="serviceName" v-model="serviceName" placeholder="例如 MySQL80" /></label>
    <label>预期端口<input name="expectedPort" v-model="port" inputmode="numeric" placeholder="可选" /></label>
    <p v-if="error" class="form-error" role="alert">{{ error }}</p>
    <p v-if="saveError" class="form-error" role="alert">{{ saveError }}</p>
    <div class="form-actions">
      <button type="button" class="button" data-testid="cancel-add" :disabled="disabled" @click="emit('cancel')">取消</button>
      <button type="submit" class="button" :disabled="disabled">保存</button>
    </div>
  </form>
</template>
