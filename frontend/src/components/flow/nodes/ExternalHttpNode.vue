<script setup>
import { inject, reactive, ref, onMounted } from "vue";
// import { ElMessageBox } from 'element-plus'
import { copyProperties, getDefaultBranch, httpReq } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const nodeSetFormVisible = ref(false);
const getNode = inject('getNode');
const { robotId } = inject('robotId');
const node = getNode();
const formLabelWidth = '100px'
const apis = reactive([])
const nodeName = ref()
const apisRef = ref()
const nodeData = reactive({
    nodeName: 'ExternalHttpNode',
    httpApiName: '',
    httpApiId: '',
    valid: false,
    invalidMessages: [],
    newNode: true,
    branches: []
})
const validate = () => {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (nodeData.httpApiName == '' || nodeData.httpApiId == '')
        m.push('Please choose a HTTP interface');
    if (getNode().getPortAt(0).id == '')
        m.push('Please connect "Next" to another node');
    d.valid = m.length == 0;
}
const saveForm = () => {
    const port = getNode().getPortAt(0);
    const branch = nodeData.branches[0];
    branch.branchName = 'Next';
    branch.branchId = port.id;
    validate();
    // console.log(this.nodeData);
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm()
}
const showOptions = (v) => {
    for (var i in apis) {
        if (apis[i].id == v) {
            nodeData.httpApiName = apis[i].name
            break
        }
    }
}
const hideForm = () => {
    nodeSetFormVisible.value = false;
}
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
onMounted(async () => {
    // console.log('httpNode')
    const t = await httpReq('GET', 'external/http', { robotId: robotId }, null, null);
    // console.log(t);
    if (t.status == 200) {
        for (var x in t.data) {
            if (t.data.hasOwnProperty(x)) {
                // console.log(t.data[x])
                apis.push(t.data[x]);
            }
        }
    }
    node.addPort({
        group: 'absolute',
        args: { x: nodeName.value.offsetWidth - 15, y: nodeName.value.offsetHeight + 50 },
        attrs: {
            text: {
                text: 'Next',
                fontSize: 12,
            },
        },

    });
    nodeData.branches.push(getDefaultBranch())
})
</script>
<style scoped>
.nodeBox {
    border: 2px #0000000e solid;
    height: 100%;
    width: 100%;
    background-color: white;
    font-size: 12px;
}

.nodeTitle {
    background-color: rgb(1, 165, 188);
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}

.optionWidth {
    width: 110px;
}
</style>
<template>
    <div class="nodeBox">
        <div ref="nodeName" class="nodeTitle">
            {{ nodeData.nodeName }}
            <span v-show="nodeData.invalidMessages.length > 0">
                <el-tooltip class="box-item" effect="dark" :content="nodeData.invalidMessages.join('<br/>')"
                    placement="bottom" raw-content>
                    <el-icon color="red" size="16">
                        <EpWarning />
                    </el-icon>
                </el-tooltip>
            </span>
        </div>
        <div>Call Http: {{ nodeData.httpApiName }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="70px" :model="nodeData" style="max-width: 850px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="HTTP APIs" :label-width="formLabelWidth">
                    <el-select ref="apisRef" v-model="nodeData.httpApiId" placeholder="Choose an http interface"
                        @change="(v) => showOptions(v)">
                        <el-option v-for="item in apis" :key="item.id" :label="item.name" :value="item.id" />
                    </el-select>
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-text size="large">
                        <div><strong>Please note</strong> that this is just calling the interface, but the returned data
                            will be
                            ignored.</div>
                        <div>If you need to obtain data, please create a variable and select a certain interface as
                            the source of the data.</div>
                        <div>Checkout tutorial.</div>
                    </el-text>
                </el-form-item>
            </el-form>
            <div class="demo-drawer__footer">
                <el-button type="primary" :loading="loading" @click="saveForm()">{{ t('lang.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('lang.common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>