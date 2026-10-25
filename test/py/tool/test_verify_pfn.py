"""Independent C-declarator oracle regression cases."""
import importlib.util
from pathlib import Path
import unittest
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location("verify_pfn", ROOT / "tool/verify_pfn.py")
ORACLE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ORACLE)


class PrototypeOracleTest(unittest.TestCase):
    def test_pointer_qualifiers_and_fixed_array_decay(self):
        cases = [
            ("const <type>char</type>* const* <name>names</name>", "*const *const std::ffi::c_char"),
            ("const <type>char</type>** <name>names</name>", "*mut *const std::ffi::c_char"),
            ("<type>void</type>** <name>data</name>", "*mut *mut std::ffi::c_void"),
            ("const <type>float</type> <name>values</name>[4]", "*const f32"),
            ("const <type>VkBuffer</type>* const <name>values</name>[4][VK_UUID_SIZE]", "*const [*const vk::Buffer; vk::UUID_SIZE as usize]"),
            ("<type>VkDevice</type> <name>device</name>", "vk::DeviceHandle"),
            ("<type>uint64_t</type> <name>size</name>", "u64"),
            ("<type>int32_t</type> <name>index</name>", "i32"),
        ]
        for source, expected in cases:
            with self.subTest(source=source):
                self.assertEqual(ORACLE.rust_type(ET.fromstring(f"<param>{source}</param>")), expected)

    def test_comments_do_not_change_declarations_or_return_types(self):
        node = ET.fromstring("<param>const <type>VkBuffer</type>* <name>values</name><comment>optional [bad]</comment></param>")
        self.assertEqual(ORACLE.declaration(node), "const VkBuffer* values")
        self.assertEqual(ORACLE.rust_type(node), "*const vk::Buffer")
        node = ET.fromstring("<proto><type>void</type> <name>vkCall</name></proto>")
        self.assertEqual(ORACLE.rust_type(node, parameter=False), "()")

    def test_pinned_inventory_includes_all_core_and_protected_commands(self):
        commands = ORACLE.admitted_commands(ET.parse(ROOT / "registry/vk.xml").getroot())
        self.assertEqual(len(commands), 842)
        self.assertIn("vkCmdBeginRendering", commands)
        self.assertIn("vkMapMemory2", commands)
        self.assertIn("vkCmdSetDispatchParametersARM", commands)
        self.assertIn("vkCreateWin32SurfaceKHR", commands)
        self.assertIs(commands["vkMapMemory2"], commands["vkMapMemory2KHR"])


if __name__ == "__main__":
    unittest.main()
