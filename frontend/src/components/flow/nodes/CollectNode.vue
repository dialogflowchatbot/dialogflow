<script setup>
import { inject, reactive, ref, onMounted } from 'vue';
import { copyProperties, httpReq, getDefaultBranch } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const nodeSetFormVisible = ref(false);
const nodeData = reactive({
    nodeName: t('lang.collectNode.nodeName'),
    collectTypeName: '',
    collectType: '',
    customizeRegex: '',
    collectSaveVarName: '',
    valid: false,
    invalidMessages: [],
    branches: [],
    newNode: true,
});
const nodeName = ref();
const getNode = inject('getNode');
const { robotId } = inject('robotId');
const node = getNode();
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
const collectionTypes = [{ label: tm('lang.collectNode.cTypes')[0], value: 'UserInput' }, { label: tm('lang.collectNode.cTypes')[1], value: 'Number' }, { label: tm('lang.collectNode.cTypes')[2], value: 'CustomizeRegex' }];
const variables = []
onMounted(async () => {
    // console.log('collectNode')
    const node = getNode();
    const data = node.getData();
    // console.log(data);
    copyProperties(data, nodeData);
    // if (data) {
    //     if (data.nodeName)
    //         nodeData.nodeName = data.nodeName;
    //     nodeData.collectType = data.collectType;
    //     nodeData.collectSaveVarName = data.collectSaveVarName;
    //     if (data.newNode)
    //         nodeData.newNode = data.newNode;
    // }
    // console.log(nodeData.newNode);
    if (nodeData.newNode) {
        nodeData.nodeName += data.nodeCnt.toString();
        const heightOffset = nodeName.value.offsetHeight + 50;
        const x = nodeName.value.offsetWidth - 15;
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            attrs: {
                text: {
                    text: tm('lang.collectNode.branches')[0],
                    fontSize: 12,
                },
            },
        });
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset + 20 },
            attrs: {
                text: {
                    text: tm('lang.collectNode.branches')[1],
                    fontSize: 12,
                },
            },
        });
        nodeData.newNode = false;
        // console.log(nodeData);
    }
    const t = await httpReq('GET', 'variable', { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200 && t.data) {
        variables.splice(0, variables.length);
        t.data.forEach(function (item, index, arr) {
            this.push({ label: item.varName, value: item.varName });
        }, variables);
    }
    validate();
});
const errors = tm('lang.collectNode.errors')
function validate() {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!d.nodeName)
        m.push(errors[0]);
    if (!d.collectType)
        m.push(errors[1]);
    if (!d.collectSaveVarName)
        m.push(errors[2]);
    if (d.branches == null || d.branches.length == 0)
        m.push(errors[3]);
    d.valid = m.length == 0;
}
function hideForm() {
    nodeSetFormVisible.value = false;
}
function saveForm() {
    setCollectTypeName();
    const node = getNode();
    const ports = node.getPorts();
    nodeData.branches.splice(0, nodeData.branches.length);
    for (let i = 0; i < ports.length; i++) {
        const branch = getDefaultBranch();
        branch.branchName = ports[i].attrs.text.text;
        branch.branchId = ports[i].id;
        branch.branchType = i == 0 ? 'InfoCollectedSuccessfully' : 'GotoAnotherNode';
        // branch.conditionGroup[0][0].conditionType = 'UserInput';
        nodeData.branches.push(branch);
    }
    validate();
    if (nodeData.collectType == 'CustomizeRegex')
        nodeData.collect_type = { "CustomizeRegex": nodeData.customizeRegex };
    else
        nodeData.collect_type = nodeData.collectType;
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm();
}
function setCollectTypeName() {
    for (let i = 0; i < collectionTypes.length; i++) {
        if (nodeData.collectType == collectionTypes[i].value) {
            nodeData.collectTypeName = collectionTypes[i].label;
        }
    }
}

const labels = tm('lang.collectNode.labels')
const formLabelWidth = '140px'
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
    background-color: rgb(90, 213, 235);
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
        <div>{{ t('lang.collectNode.cTypeName') }}: {{ nodeData.collectTypeName }}</div>
        <div>{{ t('lang.collectNode.varName') }}: {{ nodeData.collectSaveVarName }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="100px" :model="nodeData" style="max-width: 460px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item :label="labels[0]" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.collectType" :placeholder="labels[1]">
                        <el-option v-for="item in collectionTypes" :key="item.label" :label="item.label"
                            :value="item.value" />
                    </el-select>
                </el-form-item>
                <el-form-item v-show="nodeData.collectType == 'CustomizeRegex'" :label="labels[2]"
                    :label-width="formLabelWidth">
                    <el-input v-model="nodeData.customizeRegex" />
                </el-form-item>
                <el-form-item :label="labels[3]" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.collectSaveVarName" :placeholder="labels[4]">
                        <el-option v-for="item in variables" :key="item.label" :label="item.label"
                            :value="item.value" />
                    </el-select>
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