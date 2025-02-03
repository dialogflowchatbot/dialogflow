<script setup>
import { inject, reactive, ref, onMounted } from 'vue';
import { copyProperties } from '../../../assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const nodeSetFormVisible = ref(false);
const nodeData = reactive({
    nodeName: 'The end',
    endingText: '',
    valid: false,
    invalidMessages: [],
    newNode: true,
});
const nodeName = ref();
const getNode = inject('getNode');
const node = getNode();
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
onMounted(async () => {
    // const node = getNode();
    const data = node.getData();
    copyProperties(data, nodeData);
    if (nodeData.newNode) {
        nodeData.nodeName += data.nodeCnt.toString();
        nodeData.newNode = false;
    }
    validate();
});
function validate() {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (d.endingText && d.endingText.length > 10000)
        m.push('The text entered cannot exceed 10,000 characters');
    d.valid = m.length == 0;
}
function hideForm() {
    nodeSetFormVisible.value = false;
}
const nodeAnswer = ref()
function saveForm() {
    // const node = getNode();
    validate();
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm();
    const heightOffset = nodeName.value.offsetHeight + nodeAnswer.value.offsetHeight;
    console.log(heightOffset)
    node.resize(node.size().width, 20 + heightOffset, { direction: 'bottom' })
}

const formLabelWidth = '90px'
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
    background-color: rgb(34, 25, 106);
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
        <div ref="nodeAnswer" style="white-space: pre-wrap;font-size:12px;">{{ nodeData.endingText }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-position="labelPosition" label-width="100px" :model="nodeData" style="max-width: 460px">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="Ending text" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.endingText" type="textarea" />
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