<template>
  <el-tabs
    v-model="editableTabsValue"
    type="card"
    @tab-remove="removeTab"
    :before-leave="beforeLeave"
    class="my-tab-pane"
  >
    <el-tab-pane
      v-for="item in editableTabs"
      :key="item.name"
      :label="item.title"
      :name="item.name"
      closable
    >
      {{ item.content }}
    </el-tab-pane>
    <el-tab-pane key="add" name="add">
      <span
        slot="label"
        style="padding: 8px; font-size: 20px; font-weight: bold"
      >
        +
      </span>
    </el-tab-pane>
  </el-tabs>
</template>

<script>
export default {
  data() {
    return {
      editableTabsValue: "1",
      currentIndex: 1,
      editableTabs: [
        {
          title: "活动1",
          name: "1",
          content: "asdf",
        },
      ],
      tabIndex: 1,
      addIndex: 1,
    };
  },
  methods: {
    addTab() {
      let newTabIndex = ++this.tabIndex + "";
      this.editableTabs.push({
        title: "活动" + ++this.addIndex,
        name: newTabIndex,
      });
      this.editableTabsValue = newTabIndex;
      this.currentIndex = newTabIndex;
    },
    removeTab(targetName) {
      if (this.editableTabs.length <= 1) {
        return false;
      }
      var self = this;
      let tabs = self.editableTabs;
      let activeName = self.editableTabsValue;
      if (activeName === targetName) {
        tabs.forEach((tab, index) => {
          if (tab.name === targetName) {
            let nextTab = tabs[index + 1] || tabs[index - 1];
            if (nextTab) {
              activeName = nextTab.name;
            }
          }
        });
      }
      self.editableTabsValue = activeName;
      self.editableTabs = tabs.filter((tab) => tab.name !== targetName);
      self.editableTabs.map((tab, index) => {
        tab.title = "活动" + (index + 1);
        self.addIndex = index + 1;
      });
      self.currentIndex = self.editableTabsValue;
    },
    /* 活动标签切换时触发 */
    beforeLeave(currentName) {
      //重点，如果name是add，则什么都不触发
      if (currentName == "add") {
        this.addTab();
        return false;
      } else {
        this.currentIndex = currentName;
      }
    },
  },
};
</script>