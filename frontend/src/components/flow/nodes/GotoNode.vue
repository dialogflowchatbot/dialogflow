<!-- <script>
export default {
    name: 'GotoNode',
};
</script> -->
<script setup>
import { inject, reactive, ref, onMounted, nextTick } from 'vue';
import { copyProperties, httpReq } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const { robotId } = inject('robotId');
// const getGraph = inject('getGraph');
const getNode = inject('getNode');
const { subflowNames, updateSubFlowNames } = inject('subFlowNamesFn');
const nodeSetFormVisible = ref(false);
const mainFlows = ref([])
const subFlowNames = ref([])

const types18 = tm('lang.gotoNode.types');
const gotoTypes = [
    // { label: types18[0], value: 'Terminate', disabled: false },
    { label: types18[1], value: 'GotoMainFlow', disabled: false },
    { label: types18[2], value: 'GotoSubFlow', disabled: false },
    { label: types18[3], value: 'GotoExternalLink', disabled: false }
];
const nodeData = reactive({
    nodeName: t('lang.gotoNode.nodeName'),
    brief: '',
    gotoType: '',
    gotoMainFlowId: '',
    gotoSubFlowName: '',
    gotoSubFlowId: '',
    externalLink: '',
    valid: false,
    invalidMessages: [],
    newNode: true,
});
const showSubFlowOptions = ref(false)

getNode().on("change:data", ({ current }) => {
    // console.log('toggled');
    // const { name, text, nodeSetFormVisible } = current;
    nodeSetFormVisible.value = true;
});

onMounted(async () => {
    // console.log('gotoNode')
    const node = getNode();
    const data = node.getData();
    copyProperties(data, nodeData);
    selectGotoType(nodeData.goToType);
    // if (data) {
    //     if (data.nodeName)
    //         nodeData.nodeName = data.nodeName;
    //     nodeData.goToType = data.goToType;
    //     selectGotoType(nodeData.goToType);
    //     nodeData.goToSubFlowId = data.goToSubFlowId;
    //     nodeData.goToSubFlowName = data.goToSubFlowName;
    //     nodeData.brief = data.brief;
    //     nodeData.externalLink = data.externalLink;
    //     if (data.newNode)
    //         nodeData.newNode = data.newNode;
    // }
    if (nodeData.newNode)
        nodeData.nodeName += data.nodeCnt.toString();
    nodeData.newNode = false;
    validate();

    const r = await httpReq('GET', 'mainflow', { robotId: robotId, }, null, null);
    // console.log(r);
    if (r.status == 200)
        mainFlows.value = r.data;
    if (data.gotoSubFlowId) {
        await selectedMainFlow(data.gotoMainFlowId);
    }
})

async function selectGotoType(t) {
    // console.log(updateSubFlowNames());
    if ('GotoMainFlow' == t) {
    } else if ('GotoSubFlow' == t) {
        subFlowNames.value = updateSubFlowNames();
        showSubFlowOptions.value = true;
    } else {
        showSubFlowOptions.value = false;
    }
}

async function selectedMainFlow(id) {
    // console.log(id);
    if (id) {
        const r = await httpReq('GET', 'subflow/simple', { robotId: robotId, mainFlowId: id, data: '' }, null, null);
        // console.log(r);
        if (r.status == 200)
            subFlowNames.value = r.data;
    } else {
        subFlowNames.value = updateSubFlowNames();
    }
    showSubFlowOptions.value = true;
}

const errors18 = tm('lang.gotoNode.errors')
function validate() {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!d.nodeName)
        m.push(errors18[0]);
    if (!d.gotoType)
        m.push(errors18[1]);
    if (d.gotoType == 'GotoSubFlow' && !d.gotoSubFlowId)
        m.push(errors18[2]);
    if (d.gotoType == 'GotoExternalLink' && !d.externalLink)
        m.push(errors18[3]);
    d.valid = m.length == 0;
}
function hideForm() {
    nodeSetFormVisible.value = false;
}
const briefs18 = tm('lang.gotoNode.briefs')
function saveForm() {
    // console.log(nodeData.gotoType);
    nodeData.brief = briefs18[0] + ': ';
    if (nodeData.gotoType == 'Terminate')
        nodeData.brief += briefs18[1];
    else if (nodeData.gotoType == 'GotoMainFlow') {
        nodeData.brief += briefs18[4] + '\n';
        for (let i = 0; i < mainFlows.value.length; i++) {
            if (nodeData.gotoMainFlowId == mainFlows.value[i].id) {
                nodeData.brief += mainFlows.value[i].name;
                break;
            }
        }
        for (let i = 0; i < subFlowNames.value.length; i++) {
            if (nodeData.gotoSubFlowId == subFlowNames.value[i].id) {
                nodeData.gotoSubFlowName = subFlowNames.value[i].name;
                nodeData.brief += ' - ' + nodeData.gotoSubFlowName;
                break;
            }
        }
    } else if (nodeData.gotoType == 'GotoSubFlow') {
        for (let i = 0; i < subFlowNames.value.length; i++) {
            if (nodeData.gotoSubFlowId == subFlowNames.value[i].id) {
                nodeData.gotoSubFlowName = subFlowNames.value[i].name;
                break;
            }
        }
        nodeData.brief += briefs18[2] + ': ' + nodeData.gotoSubFlowName;
    } else if (nodeData.gotoType == 'GotoExternalLink')
        nodeData.brief += briefs18[3] + ': ' + nodeData.externalLink;
    const node = getNode();
    validate();
    console.log(nodeData);
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm();
}

const formLabelWidth = '130px'
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
    background-color: rgb(67, 211, 153);
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}

/* .optionWidth {
    width: 110px;
} */
</style>
<template>
    <div class="nodeBox">
        <div class="nodeTitle">
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
        <div>{{ nodeData.brief }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-if="nodeSetFormVisible" v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl"
            size="70%" :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="70px" :model="nodeData" style="max-width: 700px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item :label="t('lang.gotoNode.gotoType')" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.gotoType" :placeholder="t('lang.gotoNode.gotoTypePH')"
                        @change="selectGotoType">
                        <el-option v-for="item in gotoTypes" :key="item.label" :label="item.label" :value="item.value"
                            :disabled="item.disabled" />
                    </el-select>
                </el-form-item>
                <div v-show="nodeData.gotoType === 'GotoMainFlow'">
                    <el-form-item :label="t('lang.gotoNode.gotoMainFlow')" :label-width="formLabelWidth">
                        <el-select v-model="nodeData.gotoMainFlowId" :placeholder="t('lang.gotoNode.gotoMainFlowPH')"
                            @change="selectedMainFlow">
                            <el-option v-for="item in mainFlows" :key="item.id" :label="item.name" :value="item.id" />
                        </el-select>
                    </el-form-item>
                </div>
                <div v-show="showSubFlowOptions">
                    <el-form-item :label="t('lang.gotoNode.gotoSubFlow')" :label-width="formLabelWidth">
                        <el-select v-model="nodeData.gotoSubFlowId" :placeholder="t('lang.gotoNode.gotoSubFlowPH')">
                            <el-option v-for="item in subFlowNames" :key="item.id" :label="item.name"
                                :value="item.id" />
                        </el-select>
                    </el-form-item>
                </div>
                <div v-show="nodeData.gotoType === 'GotoExternalLink'">
                    <el-form-item :label="t('lang.gotoNode.externalLink')" :label-width="formLabelWidth">
                        <el-input v-model="nodeData.externalLink" />
                    </el-form-item>
                </div>
            </el-form>
            <div>
                <el-button type="primary" :loading="loading" @click="saveForm()">{{ t('lang.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('lang.common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>
