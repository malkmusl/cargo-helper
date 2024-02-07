use crate::rust::write_file;
use super::get_date;
use super::get_year;

pub fn generate_license(src_dir: &str, project_name: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"
## <p align="center">Malkmusl's Proprietary License Agreement</p>

#### <p align="center">Version 1.3 Revision 1</p>

##### <p align="center">Copyright (c) {copyright_year} Lucas Malkmus</p>

---

### <p align="center">Introduction</p>

Welcome to the Malkmusl's Proprietary License Agreement. This agreement outlines the terms and conditions under which you, the Licensee, may use the malkmusl's software, developed and owned by Lucas Malkmus(malkmusl). By using the software, you acknowledge and agree to abide by the terms of this license. It is important to carefully read and understand the following provisions before using the software.

---

### <p align="center">Version and Revision Explanation</p>

In this context, the term "Version" refers to incremental updates and additions made to the license text, allowing for the inclusion of new terms or modifications to existing ones. This enables us to adapt the license to changing circumstances and licensing practices while ensuring clarity and relevance over time.

Conversely, the term "Revision" is utilized solely to update the copyright information within the license, ensuring compliance with legal requirements and acknowledging the contributions of various contributors to the licensed software.

It is crucial to note that the Revision may differ from one project to another, as each project may have unique requirements, contributors, and collaborative dynamics. By maintaining this distinction between Version and Revision, we can foster a flexible licensing approach that accommodates diverse projects while upholding proper copyright management across all endeavors.

When this License undergoes sub-version updates, such as 1.0 to 1.1, 1.2 to 1.3, and others, you will find those changes presented in a clear and organized manner within this file, enhancing readability. However, please note that the version history for this file will be cleared when a major update, like 1.0, 2.0, 3.0, etc., occurs. For the full and comprehensive list of all changes, including major updates, we recommend visiting our website: malkmusl.cloud.

###### Please exercise caution while using the early stages of this license. A more stable version of this license is anticipated with the arrival of the first major update to version 2.0. We appreciate your understanding and look forward to providing you with an enhanced experience soon. 

---

### <p align="center">Terms and Conditions</p>

1. The Licensee acknowledges and agrees that their use of the software, including any contributions made, is subject to the terms and conditions of the Contributor License Agreement (CLA) with Lucas Malkmus ("malkmusl").

2. The Licensee is granted a non-exclusive, non-transferable license to use the software for their own internal purposes only, without the right to modify, distribute, or create derivative works, unless expressly permitted by the terms of the signed CLA.

3. Any modifications, distribution, or creation of derivative works of the software shall require the Licensee to sign a separate agreement or obtain explicit written permission from malkmusl.

4. The Licensee agrees not to sublicense, rent, lease, sell, or otherwise transfer their rights to the software without the prior written consent of malkmusl.

5. The Licensee acknowledges that the software is provided "as is" without any warranties or representations, express or implied. malkmusl shall not be liable for any damages arising out of the use or inability to use the software.

6. This proprietary license is in addition to, and does not replace or supersede, any other licenses or agreements between the Licensee and malkmusl.

---

### <p align="center">Why Proprietary?<p>

The Malkmusl's Proprietary License Agreement has been designed to protect the intellectual property and investments of malkmusl. By maintaining exclusive control over the software, malkmusl can safeguard his unique innovations, research, and development efforts.

This proprietary license allows malkmusl to maintain ownership of the software, granting him control over its distribution, licensing, and future development. Ownership enables ongoing investment in enhancements, bug fixes, security updates, and new features to benefit you, the Licensee, and the wider user base.

Additionally, the proprietary nature of the software enables malkmusl to provide dedicated support, professional services, and quality assurance, ensuring a reliable and stable user experience. malkmusl takes responsibility for the software's performance, security, and compatibility, reducing potential risks for you and promoting a higher level of trust.

By adhering to this proprietary license, you gain access to a well-maintained and supported software solution that reflects malkmusl's commitment to excellence and continuous improvement. Furthermore, it fosters an environment where malkmusl can sustain his investment in research, innovation, and the overall advancement of the software.

Please carefully review the following Proprietary License Clause to understand the specific terms and conditions of your use of the Malkmusl's software.

---

### <p align="center">Revision History</p> 

- **Revision 1**: Initial release for {project_name}, {date} 

---

### <p align="center">Version History</p>

- **Version 1.0**: Initial Release
    - The Licensee acknowledges and agrees that their use of the software, including any contributions made, is subject to the terms and conditions of the Contributor License Agreement (CLA) with Lucas Malkmus ("malkmusl").
    - The Licensee is granted a non-exclusive, non-transferable license to use the software for their own internal purposes only, without the right to modify, distribute, or create derivative works, unless expressly permitted by the terms of the signed CLA.
    - Any modifications, distribution, or creation of derivative works of the software shall require the Licensee to sign a separate agreement or obtain explicit written permission from malkmusl.
    - The Licensee agrees not to sublicense, rent, lease, sell, or otherwise transfer their rights to the software without the prior written consent of malkmusl.
    - The Licensee acknowledges that the software is provided "as is" without any warranties or representations, express or implied. malkmusl shall not be liable for any damages arising out of the use or inability to use the software.
    - This proprietary license is in addition to, and does not replace or supersede, any other licenses or agreements between the Licensee and malkmusl.

- **Version 1.1**: Version and Revision Explanation
    - Reformat of the License Text
    - Added a clear explanation of the distinction between "Version" and "Revision" in the license text.
    - The "Version" denotes incremental updates and additions to the license text, allowing for the inclusion of new terms or modifications to existing ones, adapting the license to changing circumstances and licensing practices over time.
    - The "Revision" is specifically used to update copyright information within the license, ensuring compliance with legal requirements and acknowledging the contributions of various contributors to the licensed software.
    - Clarified that the Revision may vary from project to project, accommodating unique requirements, contributors, and collaborative dynamics in each case. This approach fosters a flexible licensing framework while maintaining proper copyright management across all projects.
    - Added Version History

- **Version 1.2**: Section renaming
    - Renamed Section Malkmusl's Proprietary License Agreement to Terms and Conditions

- **Version 1.3**: Update to the Version and Revision Explanation Section
    - added the following phrase to the Version and Revision Explanation Section:  When this License undergoes sub-version updates, such as 1.0 to 1.1, 1.2 to 1.3, and others, you will find those changes presented in a clear and organized manner within this file, enhancing readability. However, please note that the version history for this file will be cleared when a major update, like 1.0, 2.0, 3.0, etc., occurs. For the full and comprehensive list of all changes, including major updates, we recommend visiting our website: malkmusl.cloud
    - added a note with this content to the Section: Please exercise caution while using the early stages of this license. A more stable version of this license is anticipated with the arrival of the first major update to version 2.0. We appreciate your understanding and look forward to providing you with an enhanced experience soon.

        "##,
        project_name = project_name,
        copyright_year = get_year(),
        date = get_date(),
    );

    let main_rs_path = format!("{}/LICENSE.md", src_dir);
    write_file(&main_rs_path, &main_rs_content)
}
