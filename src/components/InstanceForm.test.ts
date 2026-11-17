import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "../App.vue";

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

describe("实例登记", () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockResolvedValue([]);
  });

  it("缺少实例名称时不保存并指出具体字段", async () => {
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('form').trigger('submit');

    expect(wrapper.text()).toContain("请输入实例名称");
    expect(invoke).not.toHaveBeenCalledWith("save_instance", expect.anything());
  });

  it("缺少 Windows 服务名时不保存", async () => {
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("MySQL");
    await wrapper.find("form").trigger("submit");

    expect(wrapper.text()).toContain("请输入 Windows 服务名");
    expect(invoke).not.toHaveBeenCalledWith("save_instance", expect.anything());
  });

  it("端口超出范围时不保存", async () => {
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("MySQL");
    await wrapper.find('[name="serviceName"]').setValue("MySQL80");
    await wrapper.find('[name="expectedPort"]').setValue("65536");
    await wrapper.find("form").trigger("submit");

    expect(wrapper.text()).toContain("端口必须在 1 到 65535 之间");
    expect(invoke).not.toHaveBeenCalledWith("save_instance", expect.anything());
  });

  it("登记同名的不同 Windows 服务时分别显示服务名", async () => {
    const instances: unknown[] = [];
    invoke.mockImplementation(async (command: string, args?: { instance?: unknown }) => {
      if (command === "list_instances") return [...instances];
      if (command === "save_instance") {
        instances.push(args?.instance);
        return null;
      }
      throw new Error(`Unexpected command: ${command}`);
    });

    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("MySQL");
    await wrapper.find('[name="serviceName"]').setValue("MySQL80");
    await wrapper.find('form').trigger('submit');
    await flushPromises();
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("MySQL");
    await wrapper.find('[name="serviceName"]').setValue("MySQL57");
    await wrapper.find('form').trigger('submit');
    await flushPromises();

    expect(wrapper.text()).toContain("MySQL80");
    expect(wrapper.text()).toContain("MySQL57");
    expect(instances).toHaveLength(2);
  });

  it("保存尚未结束时不重复提交", async () => {
    invoke.mockImplementation(async (command: string) => {
      if (command === "list_instances") return [];
      if (command === "save_instance") return new Promise(() => {});
    });
    const wrapper = mount(App);
    await flushPromises();
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("Redis");
    await wrapper.find('[name="serviceName"]').setValue("Redis");
    await wrapper.find('form').trigger('submit');
    await wrapper.find('form').trigger('submit');

    expect(invoke.mock.calls.filter(([command]) => command === "save_instance")).toHaveLength(1);
    expect(wrapper.find('[data-testid="cancel-add"]').attributes('disabled')).toBeDefined();
  });

  it("空列表只显示添加入口，不显示未实现功能", async () => {
    const wrapper = mount(App);
    await flushPromises();

    expect(wrapper.text()).toContain("尚未登记服务");
    expect(wrapper.find('form').exists()).toBe(false);
    expect(wrapper.text()).not.toContain("重新扫描");
    expect(wrapper.text()).not.toContain("启动");
  });

  it("搜索匹配实例名称或 Windows 服务名并区分无结果", async () => {
    invoke.mockResolvedValue([
      { id: "1", name: "数据库", serviceName: "MySQL80", expectedPort: 3306 },
      { id: "2", name: "缓存", serviceName: "Redis", expectedPort: null },
    ]);
    const wrapper = mount(App);
    await flushPromises();

    expect(wrapper.findAll('tbody tr')).toHaveLength(2);
    expect(wrapper.text()).toContain("未设置");
    await wrapper.find('[aria-label="搜索服务实例"]').setValue("数据库");
    expect(wrapper.findAll('tbody tr')).toHaveLength(1);
    await wrapper.find('[aria-label="搜索服务实例"]').setValue("mysql");
    expect(wrapper.findAll('tbody tr')).toHaveLength(1);
    expect(wrapper.text()).toContain("数据库");
    await wrapper.find('[aria-label="搜索服务实例"]').setValue("未登记");
    expect(wrapper.text()).toContain("未找到匹配的服务");
    expect(wrapper.text()).not.toContain("尚未登记服务");
  });

  it("取消添加时关闭弹窗且不保存", async () => {
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    expect(wrapper.find('[role="dialog"]').exists()).toBe(true);
    await wrapper.find('[data-testid="cancel-add"]').trigger('click');

    expect(wrapper.find('[role="dialog"]').exists()).toBe(false);
    expect(invoke).not.toHaveBeenCalledWith("save_instance", expect.anything());
  });

  it("关闭按钮和 Escape 都可退出添加弹窗", async () => {
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[data-testid="close-add"]').trigger('click');
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false);

    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[role="dialog"]').trigger('keydown', { key: 'Escape' });
    expect(wrapper.find('[role="dialog"]').exists()).toBe(false);
  });

  it("后端保存失败时保留弹窗、输入和错误信息", async () => {
    invoke.mockImplementation(async (command: string) => {
      if (command === "list_instances") return [];
      if (command === "save_instance") throw new Error("写入失败");
    });
    const wrapper = mount(App);
    await wrapper.find('[data-testid="add-service"]').trigger('click');
    await wrapper.find('[name="name"]').setValue("MySQL");
    await wrapper.find('[name="serviceName"]').setValue("MySQL80");
    await wrapper.find('form').trigger('submit');
    await flushPromises();

    expect(wrapper.find('[role="dialog"]').exists()).toBe(true);
    expect((wrapper.find('[name="serviceName"]').element as HTMLInputElement).value).toBe("MySQL80");
    expect(wrapper.find('[role="alert"]').text()).toContain("写入失败");
  });
});
