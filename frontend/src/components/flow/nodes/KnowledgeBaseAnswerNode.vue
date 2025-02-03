<script setup>
import { inject, onMounted, reactive, ref } from 'vue';
import { copyProperties, getDefaultBranch } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const getNode = inject('getNode');
const node = getNode();
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
const nodeData = reactive({
    nodeName: 'Knowledge base answer node',
    recallThresholds: 85,
    noAnswerThenChoice: 'GotoAnotherNode',
    noAnswerThen: null,
    alternateAnswer: '',
    retrieveAnswerSources: [],
    branches: [],
    valid: false,
    invalidMessages: [],
    newNode: true,
});
const brief = ref('')
const genBrief = () => {
    let h = 'Knowledge recall thresholds: ' + nodeData.recallThresholds
    h += '%\nRetrieve answer from: ' + nodeData.retrieveAnswerSources.join(',');
    h += '\nWhen no knowledge is recalled then: ';
    if (nodeData.noAnswerThenChoice == 'GotoAnotherNode')
        h += 'Goto next node.';
    else if (nodeData.noAnswerThenChoice == 'ReturnAlternateAnswerInstead') {
        h += 'Return "' + nodeData.alternateAnswer + '" instead.';
    }
    brief.value = h;
}
const nodeName = ref();
const nodeBrief = ref()
onMounted(async () => {
    // const node = getNode();
    const data = node.getData();
    copyProperties(data, nodeData);
    genBrief();
    if (nodeData.newNode) {
        const heightOffset = nodeName.value.offsetHeight + 100;
        const x = nodeName.value.offsetWidth - 15;
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            markup: [
                { tagName: "circle", selector: "bopdy" },
                { tagName: "rect", selector: "bg" }
            ],
            attrs: {
                text: {
                    text: 'Goto next node',
                    fontSize: 12,
                },
                // https://codesandbox.io/s/port-label-viwnos?file=/src/App.tsx
                bg: {
                    ref: "text",
                    refWidth: "100%",
                    refHeight: "110%",
                    refX: "-100%",
                    refX2: -15,
                    refY: -5,
                    fill: "rgb(235,238,245)"
                }
            },
        });
        nodeData.nodeName += data.nodeCnt.toString();
        nodeData.newNode = false;
    }
    validate();
})
const validate = () => {
    nodeData.invalidMessages.splice(0, nodeData.invalidMessages.length);
    if (nodeData.noAnswerThenChoice == 'ReturnAlternateAnswerInstead' && !nodeData.alternateAnswer)
        nodeData.invalidMessages.push('Please enter an alternate answer.');
    if (nodeData.retrieveAnswerSources == null || nodeData.retrieveAnswerSources.length < 1)
        nodeData.invalidMessages.push('Please select at least one source of knowledge base answers.');
    nodeData.valid = nodeData.invalidMessages.length == 0;
}
const saveForm = () => {
    const ports = node.getPorts();
    const branch = getDefaultBranch();
    branch.branchName = ports[0].attrs.text.text;
    branch.branchId = ports[0].id;
    branch.branchType = 'GotoAnotherNode';
    nodeData.branches.splice(0, nodeData.branches.length, branch);
    delete nodeData.noAnswerThen;
    if (nodeData.noAnswerThenChoice == 'GotoAnotherNode')
        nodeData.noAnswerThen = nodeData.noAnswerThenChoice;
    else if (nodeData.noAnswerThenChoice == 'ReturnAlternateAnswerInstead')
        nodeData.noAnswerThen = { 'ReturnAlternateAnswerInstead': nodeData.alternateAnswer };
    validate()
    genBrief()
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm()
}
const hideForm = () => {
    nodeSetFormVisible.value = false
}
const formLabelWidth = '215px'
const nodeSetFormVisible = ref(false)
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
    background-color: #EFB7BA;
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
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
        <div ref="nodeBrief" style="white-space: pre-wrap;font-size:12px;">{{ brief }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="100px" :model="nodeData" style="max-width: 460px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="Knowledge recall thresholds" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.recallThresholds" :min="1" :max="100" />%
                </el-form-item>
                <el-form-item label="Retrieve answer from" :label-width="formLabelWidth">
                    <el-select multiple v-model="nodeData.retrieveAnswerSources" style="width: 240px">
                        <el-option v-for="item in ['QnA', 'Document']" :key="item" :label="item" :value="item" />
                    </el-select>
                </el-form-item>
                <el-form-item label="When no knowledge is recalled" :label-width="formLabelWidth">
                    <el-radio-group v-model="nodeData.noAnswerThenChoice">
                        <el-radio value="GotoAnotherNode">Goto the next node</el-radio>
                        <el-radio value="ReturnAlternateAnswerInstead">Return to the text below instead and stay at
                            the current node.</el-radio>
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Alternate answer" :label-width="formLabelWidth"
                    v-show="nodeData.noAnswerThenChoice == 'ReturnAlternateAnswerInstead'">
                    <el-input v-model="nodeData.alternateAnswer" placeholder="" />
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