<script setup>
// Method: GET
// POST Content-Type: application/x-www-form-urlencoded
// Content-Length: 13
// enctype: application/x-www-form-urlencoded

// enctype to multipart/form-data
import { ref, reactive, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
// import { ElMessage } from 'element-plus'
import { cloneObj, copyProperties, httpReq } from '../../assets/tools.js'
import { useI18n } from 'vue-i18n'
const { t, tm, rt } = useI18n();
const route = useRoute();
const router = useRouter();
const robotId = route.params.robotId;
const httpApiData = reactive({
  id: '',
  name: '',
  description: '',
  protocol: 'http://',
  method: 'GET',
  address: '',
  timeoutMilliseconds: '1500',
  postContentType: 'UrlEncoded',
  headers: [],
  queryParams: [],
  formData: [],
  requestBody: '',
  userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/123.0',
  asyncReq: false,
})
const param = reactive({
  name: '',
  value: '',
  valueSource: '',
})
const setFormVisible = ref(false)
const varDialogVisible = ref(false)
const dynamicTitle = ref('')
const activeName = ref('h')
const editIdx = ref(0)
const vars = reactive([])
const selectedVar = ref('')
const requestBodyRef = ref()
const apiId = route.params.id;
onMounted(async () => {
  if (apiId && apiId != 'new') {
    const t = await httpReq('GET', 'external/http/' + apiId, { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200) {
      copyProperties(t.data, httpApiData);
    }
  }
  let t = await httpReq('GET', 'variable', { robotId: robotId }, null, null);
  if (t && t.status == 200 && t.data) {
    for (var x in t.data) {
      if (t.data.hasOwnProperty(x)) {
        console.log(t.data[x])
        vars.push(t.data[x]);
      }
    }
  }
})
const newParam = () => {
  param.name = '';
  param.value = '';
  param.valueSource = 'Val';
  editIdx.value = -1;
  const p = activeName.value;
  if (p == 'h')
    dynamicTitle.value = 'Add header parameter'
  else if (p == 'q')
    dynamicTitle.value = 'Add query parameter'
  else if (p == 'f')
    dynamicTitle.value = 'Add POST parameter'
  setFormVisible.value = true;
}
const addParam = () => {
  const p = cloneObj(param);
  const idx = editIdx.value;
  if (idx > -1) {
    if (activeName.value == 'h')
      httpApiData.headers[idx] = p;
    else if (activeName.value == 'q')
      httpApiData.queryParams[idx] = p;
    else if (activeName.value == 'f')
      httpApiData.formData[idx] = p;
  } else {
    if (activeName.value == 'h')
      httpApiData.headers.push(p);
    else if (activeName.value == 'q')
      httpApiData.queryParams.push(p);
    else if (activeName.value == 'f')
      httpApiData.formData.push(p);
  }
  setFormVisible.value = false
}
const editParam = (idx) => {
  editIdx.value = idx;
  if (activeName.value == 'h')
    copyProperties(httpApiData.headers[idx], param)
  else if (activeName.value == 'q')
    copyProperties(httpApiData.queryParams[idx], param)
  else if (activeName.value == 'f')
    copyProperties(httpApiData.formData[idx], param)
  setFormVisible.value = true
}
const save = async () => {
  httpApiData.protocol = httpApiData.protocol.replace('://', '').toUpperCase();
  const t = await httpReq('POST', 'external/http/' + apiId, { robotId: robotId }, null, httpApiData);
  // console.log(t);
  if (t && t.status == 200) {
    ElMessage({
      showClose: true,
      message: 'All data has been saved.',
      type: 'success',
    });
    goBack();
  } else {
    ElMessage({
      showClose: true,
      message: 'Oops, this is something wrong.',
      type: 'error',
    })
  }
}
const insertVar = () => {
  // console.log(requestBodyRef)
  // requestBodyRef.value.focus()
  // let cursorPosition = requestBodyRef.value.selectionStart
  // console.log(cursorPosition)
  // console.log(requestBodyRef.selectionStart)
  httpApiData.requestBody += '`' + selectedVar.value + '`'
  // console.log(requestBodyRef.requestBody)
  varDialogVisible.value = false
}
const goBack = () => {
  router.push({ name: 'externalHttpApis', params: { robotId: robotId } });
}
const handleClick = (tab, event) => {
  // dynamicTitle.value = 'Add ' + tab.paneLabel + ' parameter';
  // console.log(dynamicTitle.value)
  console.log(tab, event)
}
const changeTab = (v) => {
  if (v != 'POST' && activeName.value == 'f')
    activeName.value = 'q'
}
</script>
<style scoped>
.mainBody {
  margin-left: 20px;
  margin-right: 20px;
}

.my-header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}
</style>
<template>
  <div class="mainBody">
    <el-page-header :title="t('lang.common.back')" @back="goBack">
      <template #content>
        <span class="text-large font-600 mr-3"> External HTTP API </span>
      </template>
    </el-page-header>
    <p></p>
    <el-form :model="httpApiData" label-width="90px">
      <el-form-item label="Api name">
        <el-input v-model="httpApiData.name" />
      </el-form-item>
      <el-form-item label="Description">
        <el-input v-model="httpApiData.description" maxlength="256" placeholder="Some descriptions of this API"
          show-word-limit type="textarea" />
      </el-form-item>
      <el-form-item label="Method">
        <el-select v-model="httpApiData.method" placeholder="" @change="changeTab">
          <el-option label="GET" value="GET" />
          <el-option label="POST" value="POST" />
        </el-select>
      </el-form-item>
      <el-form-item label="Protocol">
        <el-select v-model="httpApiData.protocol" placeholder="">
          <el-option label="HTTP" value="http://" />
          <el-option label="HTTPS" value="https://" />
        </el-select>
      </el-form-item>
      <el-form-item label="Address">
        <el-input v-model="httpApiData.address">
          <!-- <template #prepend>POST Http://</template> -->
          <template #prepend>{{ httpApiData.method }} {{ httpApiData.protocol }}</template>
        </el-input>
      </el-form-item>
    </el-form>
    <el-text tag="b" size="large">Advanced</el-text>
    <el-form :model="httpApiData" label-width="90px">
      <el-form-item label="Timed out">
        <el-input-number v-model="httpApiData.timeoutMilliseconds" :min="200" :max="600000" /> milliseconds
      </el-form-item>
      <el-form-item label="Parameters">
        <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
          <el-tab-pane label="Header" name="h">
            <el-table :data="httpApiData.headers" stripe style="width: 100%">
              <el-table-column prop="name" label="Parameter name" width="300" />
              <el-table-column prop="value" label="Parameter value" width="200" />
              <el-table-column fixed="right" :label="tm('lang.mainflow.table')[2]" width="270">
                <template #default="scope">
                  <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                    Edit
                  </el-button> |
                  <el-button link type="primary" size="small" @click="delApi(scope.$index, scope.row)">
                    Delete
                  </el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-button type="warning" @click="newParam">+Add header</el-button>
          </el-tab-pane>
          <el-tab-pane label="Query parameters" name="q">
            <el-table :data="httpApiData.queryParams" stripe style="width: 100%">
              <el-table-column prop="name" label="Parameter name" width="300" />
              <el-table-column prop="value" label="Parameter value" width="200" />
              <el-table-column fixed="right" :label="tm('lang.mainflow.table')[2]" width="270">
                <template #default="scope">
                  <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                    Edit
                  </el-button> |
                  <el-button link type="primary" size="small" @click="delApi(scope.$index, scope.row)">
                    Delete
                  </el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-button type="warning" @click="newParam">+Add query parameter</el-button>
          </el-tab-pane>
          <el-tab-pane label="Request body" name="f" v-if="httpApiData.method == 'POST'">
            Request body type:
            <el-radio-group v-model="httpApiData.postContentType" class="ml-4">
              <el-radio value="UrlEncoded" size="large">application/x-www-form-urlencoded</el-radio>
              <el-radio value="JSON" size="large">JSON</el-radio>
              <!-- <el-radio value="1" size="large">multipart/form-data</el-radio> -->
            </el-radio-group>
            <el-table v-if="httpApiData.postContentType == 'UrlEncoded'" :data="httpApiData.formData" stripe
              style="width: 100%">
              <el-table-column prop="name" label="Parameter name" width="300" />
              <el-table-column prop="value" label="Parameter value" width="200" />
              <el-table-column fixed="right" :label="tm('lang.mainflow.table')[2]" width="270">
                <template #default="scope">
                  <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                    Edit
                  </el-button> |
                  <el-button link type="primary" size="small" @click="delApi(scope.$index, scope.row)">
                    Delete
                  </el-button>
                </template>
              </el-table-column>
            </el-table>
            <el-button type="warning" v-if="httpApiData.postContentType == 'UrlEncoded'" @click="newParam">+Add form
              data</el-button>
            <!-- <div style="margin: 20px 0" /> -->
            <el-input ref="requestBodyRef" v-if="httpApiData.postContentType == 'JSON'"
              v-model="httpApiData.requestBody" maxlength="10240" placeholder="JSON" show-word-limit type="textarea" />
            <el-button type="warning" v-if="httpApiData.postContentType == 'JSON'"
              @click="varDialogVisible = true">+Insert
              a
              variable</el-button>
          </el-tab-pane>
        </el-tabs>
      </el-form-item>
      <el-form-item label="User agent">
        <el-input v-model="httpApiData.userAgent" />
      </el-form-item>
      <el-form-item label="Sync/Async" :label-width="formLabelWidth">
        <!-- <el-switch v-model="httpApiData.asyncReq" class="mb-2" active-text="Asynchronous" inactive-text="Synchronous" /> -->
        <input type="checkbox" id="_asyncReq_" v-model="httpApiData.asyncReq" :checked="httpApiData.asyncReq" /><label
          for="_asyncReq_">Asynchronous</label>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="save">Save</el-button>
        <el-button type="info" disabled>Test (WIP)</el-button>
        <el-button @click="goBack">Cancel</el-button>
      </el-form-item>
    </el-form>
    <el-dialog v-model="setFormVisible" width="60%">
      <template #header="{ close, titleId, titleClass }">
        <div class="my-header">
          <h4 :id="titleId" :class="titleClass">{{ dynamicTitle }}</h4>
        </div>
      </template>
      <el-form :model="param">
        <el-form-item label="Name" :label-width="formLabelWidth">
          <el-input v-model="param.name" autocomplete="off" />
        </el-form-item>
        <el-form-item label="Value" :label-width="formLabelWidth">
          <el-space size="10" spacer="-">
            <el-select v-model="param.valueSource" placeholder="" style="width:150px">
              <el-option label="Const value" value="Val" />
              <el-option label="From a variable" value="Var" />
            </el-select>
            <el-input v-if="param.valueSource == 'Val'" v-model="param.value" autocomplete="off" style="width:400px" />
            <el-select v-if="param.valueSource == 'Var'" v-model="selectedVar" placeholder="Select a varaible" style="width:400px">
              <el-option v-for="item in vars" :key="item.varName" :label="item.varName" :value="item.varName" />
            </el-select>
          </el-space>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button type="primary" :loading="loading" @click="addParam">{{ $t('lang.common.save') }}</el-button>
        <el-button @click="setFormVisible = false">{{ $t('lang.common.cancel') }}</el-button>
      </template>
    </el-dialog>
    <el-dialog v-model="varDialogVisible" title="Insert a variable" width="30%">
      <el-select v-model="selectedVar" class="m-2" placeholder="Choose a variable" size="large">
        <el-option v-for="item in vars" :key="item.varName" :label="item.varName" :value="item.varName" />
      </el-select>
      <template #footer>
        <span class="dialog-footer">
          <el-button type="primary" @click="insertVar">
            {{ t('lang.common.insert') }}
          </el-button>
          <el-button @click="varDialogVisible = false">{{ t('lang.common.cancel') }}</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>