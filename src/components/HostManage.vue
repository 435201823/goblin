<template>
  <div>
    <el-row>
      <el-col :span="1">
        <el-button @click="dialogFormVisible = true">新增会话</el-button>
      </el-col>
      <el-dialog
        title="连接地址"
        :visible.sync="dialogFormVisible"
        @close="closeDialog"
      >
        <el-form :model="form">
          <el-form-item label="名称" :label-width="formLabelWidth">
            <el-input v-model="form.session_name" autocomplete="off"></el-input>
          </el-form-item>
          <el-form-item label="地址" :label-width="formLabelWidth">
            <el-input v-model="form.addr" autocomplete="off"></el-input>
          </el-form-item>
          <el-form-item label="用户名" :label-width="formLabelWidth">
            <el-input v-model="form.username" autocomplete="off"></el-input>
          </el-form-item>
          <el-form-item label="密码" :label-width="formLabelWidth">
            <el-input
              v-model="form.password"
              autocomplete="off"
              show-password
            ></el-input>
          </el-form-item>
        </el-form>
        <div slot="footer" class="dialog-footer">
          <el-button @click="dialogFormVisible = false">取 消</el-button>
          <el-button type="primary" @click="customCommand()">确 定</el-button>
        </div>
      </el-dialog>
    </el-row>
    <el-row>
      <SessionList />
    </el-row>
  </div>
</template>

<style></style>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import SessionList from "./SessionList.vue";

export default {
  data() {
    return {
      dialogTableVisible: false,
      dialogFormVisible: false,
      form: {},
      formLabelWidth: "120px",
    };
  },
  methods: {
    customCommand() {
      invoke("create_session", {
        sessionname: this.form.session_name,
        addr: this.form.addr,
        username: this.form.username,
        password: this.form.password,
      });
      this.$data.dialogFormVisible = false;
    },
    closeDialog() {
      this.form.session_name = "";
      this.form.addr = "";
      this.form.username = "";
      this.form.password = "";
    },
  },
  components: { SessionList },
};
</script>