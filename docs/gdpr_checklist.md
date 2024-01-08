# GDPR checklist

[The GDPR checklist](https://gdpr.eu/checklist/)

## Lawful basis and transparency

- [ ] Conduct an information audit to determine what information you process and who has access to it.

> Spaced is an open-source project and guidance for anyone self-hosting is important.

<!-- Organizations that have at least 250 employees or conduct higher-risk data processing are required to keep an up-to-date and detailed list of their processing activities and be prepared to show that list to regulators upon request. The best way to demonstrate GDPR compliance is using a data protection impact assessment Organizations with fewer than 250 employees should also conduct an assessment because it will make complying with the GDPR's other requirements easier. In your list, you should include: the purposes of the processing, what kind of data you process, who has access to it in your organization, any third parties (and where they are located) that have access, what you're doing to protect the data (e.g. encryption), and when you plan to erase it (if possible). -->

- [ ] Have a legal justification for your data processing activities.

> Spaced will always operate data processing based on concent. Spaced aims to provide high granualarity over what the user can consent to being processed and in what manner the user's data is used. The user can opt out of any data processing at anytime.

<!-- Processing of data is illegal under the GDPR unless you can justify it according to one of six conditions listed in Article 6. There are other provisions related to children and special categories of personal data in Articles 7-11. Review these provisions, choose a lawful basis for processing, and document your rationale. Note that if you choose "consent" as your lawful basis, there are extra obligations, including giving data subjects the ongoing opportunity to revoke consent. If "legitimate interests" is your lawful basis, you must be able to demonstrate you have conducted a privacy impact assessment. -->

- [ ] Provide clear information about your data processing and legal justification in your privacy policy.

> Alongside a clear privacy policy Spaced will include consise control over data processing in one clear overview, including how each method processes the user's data and with whom it's shared. Spaced's source code can always be checked at anytime.

<!-- You need to tell people that you're collecting their data and why (Article 12). You should explain how the data is processed, who has access to it, and how you're keeping it safe. This information should be included in your privacy policy and provided to data subjects at the time you collect their data. It must be presented "in a concise, transparent, intelligible and easily accessible form, using clear and plain language, in particular for any information addressed specifically to a child." -->

## Data security

- [ ] Take data protection into account at all times, from the moment you begin developing a product to each time you process data.

> All user data will be processed with security the measures described in this [Risk Assessment](risk_assessment.md).

<!-- You must follow the principles of "data protection by design and by default," including implementing "appropriate technical and organizational measures" to protect data. In other words, data protection is something you now have to consider whenever you do anything with other people's personal data. You also need to make sure any processing of personal data adheres to the data protection principles outlined in Article 5. Technical measures include encryption, and organizational measures are things like limiting the amount of personal data you collect or deleting data you no longer need. The point is that it needs to be something you and your employees are always aware of. -->

- [ ] Encrypt, pseudonymize, or anonymize personal data wherever possible.

> All Spaced's user data is end-to-end encrypted and as well stored with industry standard encryption. However some features may function better when at least some parts are not stored an in encrypted form. The user must first concent before the data is not stored in encrypted form.

<!-- Most of the productivity tools used by businesses are now available with end-to-end encryption built in, including email, messaging, notes, and cloud storage. The GDPR requires organizations to use encryption or pseudeonymization whenever feasible. -->

- [ ] Create an internal security policy for your team members, and build awareness about data protection.

> Spaced will have a [SECURITY](../SECURITY.md) policy for reporting vulnerabilities and who has access to releases. Spaced will keep user who self-host up to date with any risks mentioned in the security guide.

<!-- Even if your technical security is strong, operational security can still be a weak link. Create a security policy that ensures your team members are knowledgeable about data security. It should include guidance about email security, passwords, two-factor authentication, device encryption, and VPNs. Employees who have access to personal data and non-technical employees should receive extra training in the requirements of the GDPR. -->

- [ ] Know when to conduct a data protection impact assessment, and have a process in place to carry it out.

>

- [ ] Have a process in place to notify the authorities and your data subjects in the event of a data breach.

## Accountability and governance

- [ ] Designate someone responsible for ensuring GDPR compliance across your organization.
- [ ] Sign a data processing agreement between your organization and any third parties that process personal data on your behalf.
- [ ] If your organization is outside the EU, appoint a representative within one of the EU member states.
- [ ] Appoint a Data Protection Officer (if necessary)

## Privacy rights

- [ ] It's easy for your customers to request and receive all the information you have about them.
- [ ] It's easy for your customers to correct or update inaccurate or incomplete information.
- [ ] It's easy for your customers to request to have their personal data deleted.
- [ ] It's easy for your customers to ask you to stop processing their data.
- [ ] It's easy for your customers to receive a copy of their personal data in a format that can be easily transferred to another company.
- [ ] It's easy for your customers to object to you processing their data.
- [ ] If you make decisions about people based on automated processes, you have a procedure to protect their rights.

<!-- Making Spaced GDPR-proof, it is (at minimum) required to have the following in place:

Personal data is identified and processed in a fair way (meaning that you do not process data for any purpose other and the legitimate purposes)
Personal data is well managed and protected from data breaches
Personal data can be updated but also deleted from your system
Personal data is saved such that it is clear where it is stored (in case you need to update it or remove it from the system). -->
