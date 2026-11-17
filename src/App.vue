<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from "vue";
import InstanceForm from "./components/InstanceForm.vue";
import { listInstances, saveInstance } from "./api";
import type { ServiceInstance, ServiceInstanceDraft } from "./types";

const instances = ref<ServiceInstance[]>([]);
const query = ref("");
const loadError = ref("");
const saveError = ref("");
const dialogOpen = ref(false);
const saving = ref(false);
const addButton = ref<HTMLButtonElement | null>(null);
const dialog = ref<HTMLElement | null>(null);

const visibleInstances = computed(() => {
  const term = query.value.trim().toLocaleLowerCase();
  if (!term) return instances.value;
  return instances.value.filter((instance) =>
    [instance.name, instance.serviceName ?? ""].some((value) =>
      value.toLocaleLowerCase().includes(term),
    ),
  );
});

async function refresh() {
  try {
    instances.value = await listInstances();
    loadError.value = "";
  } catch (error) {
    loadError.value = `读取实例失败：${String(error)}`;
  }
}

async function openDialog() {
  saveError.value = "";
  dialogOpen.value = true;
  await nextTick();
  dialog.value?.querySelector<HTMLInputElement>('input[name="name"]')?.focus();
}

function closeDialog() {
  if (saving.value) return;
  dialogOpen.value = false;
  nextTick(() => addButton.value?.focus());
}

async function save(instance: ServiceInstanceDraft) {
  if (saving.value) return;
  saving.value = true;
  saveError.value = "";
  try {
    await saveInstance(instance);
    dialogOpen.value = false;
    await nextTick();
    addButton.value?.focus();
    await refresh();
  } catch (error) {
    saveError.value = `保存实例失败：${String(error)}`;
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <div class="page-shell" :inert="dialogOpen">
    <header class="top-bar">
      <h1>DevDesk</h1>
      <button ref="addButton" data-testid="add-service" type="button" class="button" @click="openDialog">
        添加服务
      </button>
    </header>

    <main class="content">
      <div class="section-heading">
        <h2>服务实例</h2>
        <input
          v-if="instances.length > 0"
          v-model="query"
          aria-label="搜索服务实例"
          type="search"
          placeholder="搜索实例名称或服务名"
        />
      </div>

      <p v-if="loadError" class="notice" role="alert">{{ loadError }}</p>
      <div v-else-if="instances.length === 0" class="empty-state">
        <p>尚未登记服务。</p>
        <button type="button" class="button" @click="openDialog">添加服务</button>
      </div>
      <p v-else-if="visibleInstances.length === 0" class="empty-state">未找到匹配的服务。</p>
      <div v-else class="table-scroll">
        <table>
          <thead>
            <tr>
              <th scope="col">实例名称</th>
              <th scope="col">Windows 服务名</th>
              <th scope="col">预期端口</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="instance in visibleInstances" :key="instance.id">
              <td class="instance-name">{{ instance.name }}</td>
              <td>{{ instance.serviceName }}</td>
              <td class="port">{{ instance.expectedPort ?? "未设置" }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </main>
  </div>

  <div v-if="dialogOpen" class="dialog-backdrop">
    <section
      ref="dialog"
      class="dialog"
      role="dialog"
      aria-modal="true"
      aria-labelledby="add-dialog-title"
      @keydown.esc="closeDialog"
    >
      <div class="dialog-heading">
        <h2 id="add-dialog-title">添加服务</h2>
        <button
          type="button"
          class="close-button"
          data-testid="close-add"
          aria-label="关闭添加服务"
          :disabled="saving"
          @click="closeDialog"
        >
          ×
        </button>
      </div>
      <InstanceForm :disabled="saving" :save-error="saveError" @save="save" @cancel="closeDialog" />
    </section>
  </div>
</template>

<style>
:root {
  font-family: "Microsoft YaHei UI", "Segoe UI", sans-serif;
  color: #242830;
  background: #f5f6f7;
  color-scheme: light;
}

* { box-sizing: border-box; }
body { margin: 0; }
button, input { font: inherit; }
button { cursor: pointer; }
button:disabled { cursor: default; opacity: 0.6; }
:focus-visible { outline: 2px solid #59636f; outline-offset: 2px; }

.page-shell { max-width: 1200px; min-height: 100vh; margin: 0 auto; background: #fff; }
.top-bar { height: 66px; padding: 0 32px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid #dfe2e5; background: #f8f9fa; }
.top-bar h1 { margin: 0; font-size: 20px; font-weight: 600; letter-spacing: 0; }
.button { min-height: 34px; padding: 6px 14px; border: 1px solid #cbd0d5; border-radius: 4px; background: #f1f2f3; color: #242830; }
.button:hover:not(:disabled) { background: #e7e9eb; }
.content { padding: 30px 32px 48px; }
.section-heading { min-height: 40px; margin-bottom: 22px; display: flex; align-items: center; justify-content: space-between; gap: 20px; }
.section-heading h2 { margin: 0; font-size: 18px; font-weight: 600; }
.section-heading input { width: 270px; max-width: 100%; height: 34px; padding: 6px 10px; border: 1px solid #cbd0d5; border-radius: 4px; background: #fff; color: inherit; }
.notice { margin: 12px 0; color: #a4352a; }
.empty-state { padding: 36px 0; border-top: 1px solid #dfe2e5; color: #626a73; }
.empty-state p { margin: 0 0 14px; }
.table-scroll { overflow-x: auto; }
table { width: 100%; min-width: 520px; border-collapse: collapse; table-layout: fixed; text-align: left; }
th, td { padding: 13px 16px; border-bottom: 1px solid #e2e5e8; overflow-wrap: anywhere; vertical-align: top; }
th { border-top: 1px solid #d7dce0; background: #f7f8f9; color: #4c555f; font-weight: 600; }
th:first-child { width: 40%; }
th:nth-child(2) { width: 40%; }
th:last-child { width: 20%; }
.instance-name { font-weight: 600; }
.port { white-space: nowrap; color: #555e68; }

.dialog-backdrop { position: fixed; inset: 0; z-index: 10; display: flex; align-items: center; justify-content: center; padding: 20px; background: rgba(31, 38, 45, 0.36); }
.dialog { width: min(440px, 100%); padding: 0 24px 24px; border: 1px solid #cbd0d5; border-radius: 6px; background: #fff; box-shadow: 0 12px 28px rgba(25, 30, 36, 0.16); }
.dialog-heading { min-height: 62px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid #e2e5e8; }
.dialog-heading h2 { margin: 0; font-size: 17px; font-weight: 600; }
.close-button { width: 30px; height: 30px; padding: 0; border: 0; background: transparent; color: #5c6570; font-size: 24px; line-height: 1; }
.close-button:hover:not(:disabled) { color: #242830; }
.instance-form { display: grid; gap: 16px; padding-top: 20px; }
.instance-form label { display: grid; gap: 7px; font-size: 13px; color: #424b55; }
.instance-form input { width: 100%; height: 36px; padding: 6px 10px; border: 1px solid #cbd0d5; border-radius: 4px; background: #fff; color: #242830; }
.form-error { margin: 0; color: #a4352a; font-size: 13px; }
.form-actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px; }
@media (max-width: 600px) {
  .top-bar { padding: 0 18px; }
  .content { padding: 24px 18px; }
  .section-heading { align-items: stretch; flex-direction: column; }
  .section-heading input { width: 100%; }
  th, td { padding: 11px 12px; }
}
</style>
